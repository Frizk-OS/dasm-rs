// Copyright 2012 the V8 project authors. All rights reserved.
// Copyright (C) 2026 The AOSP and FrizkOS.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
//       notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
//       copyright notice, this list of conditions and the following
//       disclaimer in the documentation and/or other materials provided
//       with the distribution.
//     * Neither the name of Google Inc. nor the names of its
//       contributors may be used to endorse or promote products derived
//       from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

export {};

interface BenchmarkRunner {
  NotifyStart?: (name: string) => void;
  NotifyError?: (name: string, error: unknown) => void;
  NotifyResult?: (name: string, result: string) => void;
  NotifyScore?: (score: string) => void;
  NotifyStep?: (name: string) => void;
}

interface BenchmarkData {
  runs: number;
  elapsed: number;
}

type Continuation = (() => Continuation | null) | null;

class Benchmark {
  name: string;
  run: () => void;
  Setup: () => void;
  TearDown: () => void;
  minIterations: number;

  constructor(
    name: string,
    run: () => void,
    setup?: () => void,
    tearDown?: () => void,
    minIterations?: number
  ) {
    this.name = name;
    this.run = run;
    this.Setup = setup ?? (() => {});
    this.TearDown = tearDown ?? (() => {});
    this.minIterations = minIterations ?? 32;
  }
}

class BenchmarkResult {
  benchmark: Benchmark;
  time: number;

  constructor(benchmark: Benchmark, time: number) {
    this.benchmark = benchmark;
    this.time = time;
  }

  valueOf(): number {
    return this.time;
  }
}

class BenchmarkSuite {
  static suites: BenchmarkSuite[] = [];
  static scores: number[] = [];
  static version = '8';

  name: string;
  reference: number;
  benchmarks: Benchmark[];
  results: BenchmarkResult[] = [];
  runner?: BenchmarkRunner;

  constructor(name: string, reference: number, benchmarks: Benchmark[]) {
    this.name = name;
    this.reference = reference;
    this.benchmarks = benchmarks;
    BenchmarkSuite.suites.push(this);
  }

  static RunSuites(runner: BenchmarkRunner): void {
    let continuation: Continuation = null;
    const suites = BenchmarkSuite.suites;
    const length = suites.length;
    BenchmarkSuite.scores = [];
    let index = 0;

    function RunStep(): void {
      while (continuation || index < length) {
        if (continuation) {
          continuation = continuation();
        } else {
          const suite = suites[index++];
          if (runner.NotifyStart) runner.NotifyStart(suite.name);
          continuation = suite.RunStep(runner);
        }
        if (continuation && typeof window !== 'undefined' && window.setTimeout) {
          window.setTimeout(RunStep, 25);
          return;
        }
      }
      if (runner.NotifyScore) {
        const score = BenchmarkSuite.GeometricMean(BenchmarkSuite.scores);
        const formatted = BenchmarkSuite.FormatScore(100 * score);
        runner.NotifyScore(formatted);
      }
    }
    RunStep();
  }

  static CountBenchmarks(): number {
    let result = 0;
    const suites = BenchmarkSuite.suites;
    for (let i = 0; i < suites.length; i++) {
      result += suites[i].benchmarks.length;
    }
    return result;
  }

  static GeometricMean(numbers: ArrayLike<number | BenchmarkResult>): number {
    let log = 0;
    for (let i = 0; i < numbers.length; i++) {
      log += Math.log(Number(numbers[i]));
    }
    return Math.pow(Math.E, log / numbers.length);
  }

  static FormatScore(value: number): string {
    if (value > 100) {
      return value.toFixed(0);
    } else {
      return value.toPrecision(3);
    }
  }

  NotifyStep(result: BenchmarkResult): void {
    this.results.push(result);
    if (this.runner?.NotifyStep) this.runner.NotifyStep(result.benchmark.name);
  }

  NotifyResult(): void {
    const mean = BenchmarkSuite.GeometricMean(this.results);
    const score = this.reference / mean;
    BenchmarkSuite.scores.push(score);
    if (this.runner?.NotifyResult) {
      const formatted = BenchmarkSuite.FormatScore(100 * score);
      this.runner.NotifyResult(this.name, formatted);
    }
  }

  NotifyError(error: unknown): void {
    if (this.runner?.NotifyError) {
      this.runner.NotifyError(this.name, error);
    }
    if (this.runner?.NotifyStep) {
      this.runner.NotifyStep(this.name);
    }
  }

  RunSingleBenchmark(benchmark: Benchmark, data?: BenchmarkData | null): BenchmarkData | null {
    function Measure(dataObj: BenchmarkData | null): void {
      let elapsed = 0;
      const start = Number(new Date());
      let n = 0;
      for (; elapsed < 1000; n++) {
        benchmark.run();
        elapsed = Number(new Date()) - start;
      }
      if (dataObj != null) {
        dataObj.runs += n;
        dataObj.elapsed += elapsed;
      }
    }

    if (data == null) {
      Measure(null);
      return { runs: 0, elapsed: 0 };
    } else {
      Measure(data);
      if (data.runs < benchmark.minIterations) return data;
      const usec = (data.elapsed * 1000) / data.runs;
      this.NotifyStep(new BenchmarkResult(benchmark, usec));
      return null;
    }
  }

  RunStep(runner: BenchmarkRunner): Continuation {
    this.results = [];
    this.runner = runner;
    const length = this.benchmarks.length;
    let index = 0;
    const suite = this;
    let data: BenchmarkData | null = null;

    function RunNextSetup(): Continuation {
      if (index < length) {
        try {
          suite.benchmarks[index].Setup();
        } catch (e) {
          suite.NotifyError(e);
          return null;
        }
        return RunNextBenchmark;
      }
      suite.NotifyResult();
      return null;
    }

    function RunNextBenchmark(): Continuation {
      try {
        data = suite.RunSingleBenchmark(suite.benchmarks[index], data);
      } catch (e) {
        suite.NotifyError(e);
        return null;
      }
      return (data == null) ? RunNextTearDown : RunNextBenchmark();
    }

    function RunNextTearDown(): Continuation {
      try {
        suite.benchmarks[index++].TearDown();
      } catch (e) {
        suite.NotifyError(e);
        return null;
      }
      return RunNextSetup;
    }

    return RunNextSetup();
  }
}

const globalScope = (typeof globalThis !== 'undefined'
  ? globalThis
  : typeof window !== 'undefined'
  ? window
  : Function('return this')()) as unknown as Record<string, unknown>;

globalScope.Benchmark = Benchmark;
globalScope.BenchmarkResult = BenchmarkResult;
globalScope.BenchmarkSuite = BenchmarkSuite;
if (typeof window !== 'undefined') {
  Object.assign(window, { Benchmark, BenchmarkResult, BenchmarkSuite });
}

if (typeof globalScope.alert !== 'undefined') {
  globalScope.alert = function(s: string): never {
    throw 'Alert called with argument: ' + s;
  };
}

Math.random = (function() {
  let seed = 49734321;
  return function(): number {
    seed = ((seed + 0x7ed55d16) + (seed << 12)) & 0xffffffff;
    seed = ((seed ^ 0xc761c23c) ^ (seed >>> 19)) & 0xffffffff;
    seed = ((seed + 0x165667b1) + (seed << 5)) & 0xffffffff;
    seed = ((seed + 0xd3a2646c) ^ (seed << 9)) & 0xffffffff;
    seed = ((seed + 0xfd7046c5) + (seed << 3)) & 0xffffffff;
    seed = ((seed ^ 0xb55a4f09) ^ (seed >>> 16)) & 0xffffffff;
    return (seed & 0xfffffff) / 0x10000000;
  };
})();
