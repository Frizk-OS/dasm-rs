// Copyright 2012 the V8 project authors. All rights reserved.
// Copyright 2009 Oliver Hunt <http://nerget.com>
// Copyright (C) 2026 The AOSP and FrizkOS.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.

export {};

declare function alert(message?: string): void;

declare class BenchmarkSuite {
  constructor(name: string, reference: number, benchmarks: Benchmark[]);
}

declare class Benchmark {
  constructor(
    name: string,
    run: () => void,
    setup?: () => void,
    tearDown?: () => void,
    minIterations?: number
  );
}

interface DisplayCallback {
  (field: Field): void;
}

interface UICallback {
  (field: Field): void;
}

class Field {
  private dens: number[];
  private u: number[];
  private v: number[];
  private rowSize: number;
  private width_: number;
  private height_: number;

  constructor(dens: number[], u: number[], v: number[], rowSize: number, width: number, height: number) {
    this.dens = dens;
    this.u = u;
    this.v = v;
    this.rowSize = rowSize;
    this.width_ = width;
    this.height_ = height;
  }

  setDensity(x: number, y: number, d: number): void {
    this.dens[(x + 1) + (y + 1) * this.rowSize] = d;
  }

  getDensity(x: number, y: number): number {
    return this.dens[(x + 1) + (y + 1) * this.rowSize];
  }

  setVelocity(x: number, y: number, xv: number, yv: number): void {
    this.u[(x + 1) + (y + 1) * this.rowSize] = xv;
    this.v[(x + 1) + (y + 1) * this.rowSize] = yv;
  }

  getXVelocity(x: number, y: number): number {
    return this.u[(x + 1) + (y + 1) * this.rowSize];
  }

  getYVelocity(x: number, y: number): number {
    return this.v[(x + 1) + (y + 1) * this.rowSize];
  }

  width(): number {
    return this.width_;
  }

  height(): number {
    return this.height_;
  }
}

class FluidField {
  private iterations = 10;
  private visc = 0.5;
  private dt = 0.1;
  private dens: number[] = [];
  private dens_prev: number[] = [];
  private u: number[] = [];
  private u_prev: number[] = [];
  private v: number[] = [];
  private v_prev: number[] = [];
  private width = 0;
  private height = 0;
  private rowSize = 0;
  private size = 0;
  private displayFunc: DisplayCallback = () => {};
  private uiCallback: UICallback = () => {};

  constructor(canvas: unknown = null) {
    this.setResolution(64, 64);
  }

  private addFields(x: number[], s: number[], dt: number): void {
    for (let i = 0; i < this.size; i++) x[i] += dt * s[i];
  }

  private set_bnd(b: number, x: number[]): void {
    let i = 0;
    if (b === 1) {
      for (i = 1; i <= this.width; i++) {
        x[i] = x[i + this.rowSize];
        x[i + (this.height + 1) * this.rowSize] = x[i + this.height * this.rowSize];
      }

      // Preserving original benchmark loop semantics where loop condition uses i from previous loop
      for (let j = 1; i <= this.height; i++) {
        x[j * this.rowSize] = -x[1 + j * this.rowSize];
        x[(this.width + 1) + j * this.rowSize] = -x[this.width + j * this.rowSize];
      }
    } else if (b === 2) {
      for (i = 1; i <= this.width; i++) {
        x[i] = -x[i + this.rowSize];
        x[i + (this.height + 1) * this.rowSize] = -x[i + this.height * this.rowSize];
      }

      for (let j = 1; j <= this.height; j++) {
        x[j * this.rowSize] = x[1 + j * this.rowSize];
        x[(this.width + 1) + j * this.rowSize] = x[this.width + j * this.rowSize];
      }
    } else {
      for (i = 1; i <= this.width; i++) {
        x[i] = x[i + this.rowSize];
        x[i + (this.height + 1) * this.rowSize] = x[i + this.height * this.rowSize];
      }

      for (let j = 1; j <= this.height; j++) {
        x[j * this.rowSize] = x[1 + j * this.rowSize];
        x[(this.width + 1) + j * this.rowSize] = x[this.width + j * this.rowSize];
      }
    }
    const maxEdge = (this.height + 1) * this.rowSize;
    x[0] = 0.5 * (x[1] + x[this.rowSize]);
    x[maxEdge] = 0.5 * (x[1 + maxEdge] + x[this.height * this.rowSize]);
    x[this.width + 1] = 0.5 * (x[this.width] + x[(this.width + 1) + this.rowSize]);
    x[(this.width + 1) + maxEdge] = 0.5 * (x[this.width + maxEdge] + x[(this.width + 1) + this.height * this.rowSize]);
  }

  private lin_solve(b: number, x: number[], x0: number[], a: number, c: number): void {
    if (a === 0 && c === 1) {
      for (let j = 1; j <= this.height; j++) {
        let currentRow = j * this.rowSize;
        ++currentRow;
        for (let i = 0; i < this.width; i++) {
          x[currentRow] = x0[currentRow];
          ++currentRow;
        }
      }
      this.set_bnd(b, x);
    } else {
      const invC = 1 / c;
      for (let k = 0; k < this.iterations; k++) {
        for (let j = 1; j <= this.height; j++) {
          let lastRow = (j - 1) * this.rowSize;
          let currentRow = j * this.rowSize;
          let nextRow = (j + 1) * this.rowSize;
          let lastX = x[currentRow];
          ++currentRow;
          for (let i = 1; i <= this.width; i++) {
            lastX = x[currentRow] = (x0[currentRow] + a * (lastX + x[++currentRow] + x[++lastRow] + x[++nextRow])) * invC;
          }
        }
        this.set_bnd(b, x);
      }
    }
  }

  private diffuse(b: number, x: number[], x0: number[], dt: number): void {
    const a = 0;
    this.lin_solve(b, x, x0, a, 1 + 4 * a);
  }

  private lin_solve2(x: number[], x0: number[], y: number[], y0: number[], a: number, c: number): void {
    if (a === 0 && c === 1) {
      for (let j = 1; j <= this.height; j++) {
        let currentRow = j * this.rowSize;
        ++currentRow;
        for (let i = 0; i < this.width; i++) {
          x[currentRow] = x0[currentRow];
          y[currentRow] = y0[currentRow];
          ++currentRow;
        }
      }
      this.set_bnd(1, x);
      this.set_bnd(2, y);
    } else {
      const invC = 1 / c;
      for (let k = 0; k < this.iterations; k++) {
        for (let j = 1; j <= this.height; j++) {
          let lastRow = (j - 1) * this.rowSize;
          let currentRow = j * this.rowSize;
          let nextRow = (j + 1) * this.rowSize;
          let lastX = x[currentRow];
          let lastY = y[currentRow];
          ++currentRow;
          for (let i = 1; i <= this.width; i++) {
            lastX = x[currentRow] = (x0[currentRow] + a * (lastX + x[currentRow] + x[lastRow] + x[nextRow])) * invC;
            lastY = y[currentRow] = (y0[currentRow] + a * (lastY + y[++currentRow] + y[++lastRow] + y[++nextRow])) * invC;
          }
        }
        this.set_bnd(1, x);
        this.set_bnd(2, y);
      }
    }
  }

  private diffuse2(x: number[], x0: number[], y: number[], y0: number[], dt: number): void {
    const a = 0;
    this.lin_solve2(x, x0, y, y0, a, 1 + 4 * a);
  }

  private advect(b: number, d: number[], d0: number[], u: number[], v: number[], dt: number): void {
    const Wdt0 = dt * this.width;
    const Hdt0 = dt * this.height;
    const Wp5 = this.width + 0.5;
    const Hp5 = this.height + 0.5;
    for (let j = 1; j <= this.height; j++) {
      let pos = j * this.rowSize;
      for (let i = 1; i <= this.width; i++) {
        let x = i - Wdt0 * u[++pos];
        let y = j - Hdt0 * v[pos];
        if (x < 0.5) x = 0.5;
        else if (x > Wp5) x = Wp5;
        const i0 = x | 0;
        const i1 = i0 + 1;
        if (y < 0.5) y = 0.5;
        else if (y > Hp5) y = Hp5;
        const j0 = y | 0;
        const j1 = j0 + 1;
        const s1 = x - i0;
        const s0 = 1 - s1;
        const t1 = y - j0;
        const t0 = 1 - t1;
        const row1 = j0 * this.rowSize;
        const row2 = j1 * this.rowSize;
        d[pos] = s0 * (t0 * d0[i0 + row1] + t1 * d0[i0 + row2]) + s1 * (t0 * d0[i1 + row1] + t1 * d0[i1 + row2]);
      }
    }
    this.set_bnd(b, d);
  }

  private project(u: number[], v: number[], p: number[], div: number[]): void {
    const h = -0.5 / Math.sqrt(this.width * this.height);
    for (let j = 1; j <= this.height; j++) {
      const row = j * this.rowSize;
      let previousRow = (j - 1) * this.rowSize;
      let prevValue = row - 1;
      let currentRow = row;
      let nextValue = row + 1;
      let nextRow = (j + 1) * this.rowSize;
      for (let i = 1; i <= this.width; i++) {
        div[++currentRow] = h * (u[++nextValue] - u[++prevValue] + v[++nextRow] - v[++previousRow]);
        p[currentRow] = 0;
      }
    }
    this.set_bnd(0, div);
    this.set_bnd(0, p);

    this.lin_solve(0, p, div, 1, 4);
    const wScale = 0.5 * this.width;
    const hScale = 0.5 * this.height;
    for (let j = 1; j <= this.height; j++) {
      let prevPos = j * this.rowSize - 1;
      let currentPos = j * this.rowSize;
      let nextPos = j * this.rowSize + 1;
      let prevRow = (j - 1) * this.rowSize;
      let currentRow = j * this.rowSize;
      let nextRow = (j + 1) * this.rowSize;

      for (let i = 1; i <= this.width; i++) {
        u[++currentPos] -= wScale * (p[++nextPos] - p[++prevPos]);
        v[currentPos] -= hScale * (p[++nextRow] - p[++prevRow]);
      }
    }
    this.set_bnd(1, u);
    this.set_bnd(2, v);
  }

  private dens_step(x: number[], x0: number[], u: number[], v: number[], dt: number): void {
    this.addFields(x, x0, dt);
    this.diffuse(0, x0, x, dt);
    this.advect(0, x, x0, u, v, dt);
  }

  private vel_step(u: number[], v: number[], u0: number[], v0: number[], dt: number): void {
    this.addFields(u, u0, dt);
    this.addFields(v, v0, dt);
    let temp = u0; u0 = u; u = temp;
    temp = v0; v0 = v; v = temp;
    this.diffuse2(u, u0, v, v0, dt);
    this.project(u, v, u0, v0);
    temp = u0; u0 = u; u = temp;
    temp = v0; v0 = v; v = temp;
    this.advect(1, u, u0, u0, v0, dt);
    this.advect(2, v, v0, u0, v0, dt);
    this.project(u, v, u0, v0);
  }

  private queryUI(d: number[], u: number[], v: number[]): void {
    for (let i = 0; i < this.size; i++) {
      u[i] = v[i] = d[i] = 0.0;
    }
    this.uiCallback(new Field(d, u, v, this.rowSize, this.width, this.height));
  }

  update(): void {
    this.queryUI(this.dens_prev, this.u_prev, this.v_prev);
    this.vel_step(this.u, this.v, this.u_prev, this.v_prev, this.dt);
    this.dens_step(this.dens, this.dens_prev, this.u, this.v, this.dt);
    this.displayFunc(new Field(this.dens, this.u, this.v, this.rowSize, this.width, this.height));
  }

  setDisplayFunction(func: DisplayCallback): void {
    this.displayFunc = func;
  }

  iterationsCount(): number {
    return this.iterations;
  }

  setIterations(iters: number): void {
    if (iters > 0 && iters <= 100) this.iterations = iters;
  }

  setUICallback(callback: UICallback): void {
    this.uiCallback = callback;
  }

  reset(): void {
    this.rowSize = this.width + 2;
    this.size = (this.width + 2) * (this.height + 2);
    this.dens = new Array(this.size);
    this.dens_prev = new Array(this.size);
    this.u = new Array(this.size);
    this.u_prev = new Array(this.size);
    this.v = new Array(this.size);
    this.v_prev = new Array(this.size);
    for (let i = 0; i < this.size; i++) {
      this.dens_prev[i] = this.u_prev[i] = this.v_prev[i] = this.dens[i] = this.u[i] = this.v[i] = 0;
    }
  }

  getDens(): number[] {
    return this.dens;
  }

  setResolution(hRes: number, wRes: number): boolean {
    const res = wRes * hRes;
    if (res > 0 && res < 1000000 && (wRes !== this.width || hRes !== this.height)) {
      this.width = wRes;
      this.height = hRes;
      this.reset();
      return true;
    }
    return false;
  }
}

let solver: FluidField | null = null;
let nsFrameCounter = 0;
let checkSumResult = 0;

function runNavierStokes(): void {
  solver?.update();
  nsFrameCounter++;
  if (nsFrameCounter === 15 && solver) {
    checkResult(solver.getDens());
  }
}

function checkResult(dens: number[]): void {
  checkSumResult = 0;
  for (let i = 7000; i < 7100; i++) {
    checkSumResult += ~~((dens[i] * 10));
  }
  if (checkSumResult !== 74) {
    if (typeof alert !== "undefined") {
      alert("checksum failed: " + checkSumResult);
    } else {
      throw new Error("checksum failed: " + checkSumResult);
    }
  }
}

function setupNavierStokes(): void {
  solver = new FluidField(null);
  solver.setResolution(128, 128);
  solver.setIterations(20);
  solver.setDisplayFunction(() => {});
  solver.setUICallback(prepareFrame);
  solver.reset();
}

function tearDownNavierStokes(): void {
  solver = null;
}

function addPoints(field: Field): void {
  const n = 64;
  for (let i = 1; i <= n; i++) {
    field.setVelocity(i, i, n, n);
    field.setDensity(i, i, 5);
    field.setVelocity(i, n - i, -n, -n);
    field.setDensity(i, n - i, 20);
    field.setVelocity(128 - i, n + i, -n, -n);
    field.setDensity(128 - i, n + i, 30);
  }
}

let framesTillAddingPoints = 0;
let framesBetweenAddingPoints = 5;

function prepareFrame(field: Field): void {
  if (framesTillAddingPoints === 0) {
    addPoints(field);
    framesTillAddingPoints = framesBetweenAddingPoints;
    framesBetweenAddingPoints++;
  } else {
    framesTillAddingPoints--;
  }
}

const NavierStokes = new BenchmarkSuite("NavierStokes", 1484000, [
  new Benchmark("NavierStokes", runNavierStokes, setupNavierStokes, tearDownNavierStokes, 16)
]);

const globalScope = (typeof globalThis !== "undefined"
  ? globalThis
  : typeof window !== "undefined"
  ? window
  : Function("return this")()) as unknown as Record<string, unknown>;

globalScope.NavierStokes = NavierStokes;
globalScope.FluidField = FluidField;
