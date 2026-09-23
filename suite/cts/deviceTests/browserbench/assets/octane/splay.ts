// Copyright 2009 the V8 project authors. All rights reserved.
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

interface SplayPayloadLeaf {
  array: number[];
  string: string;
}

interface SplayPayloadBranch {
  left: SplayPayload;
  right: SplayPayload;
}

type SplayPayload = SplayPayloadLeaf | SplayPayloadBranch;

class SplayTreeNode<T = SplayPayload> {
  key: number;
  value: T;
  left: SplayTreeNode<T> | null = null;
  right: SplayTreeNode<T> | null = null;

  constructor(key: number, value: T) {
    this.key = key;
    this.value = value;
  }

  traverse_(f: (node: SplayTreeNode<T>) => void): void {
    let current: SplayTreeNode<T> | null = this;
    while (current) {
      const left: SplayTreeNode<T> | null = current.left;
      if (left) left.traverse_(f);
      f(current);
      current = current.right;
    }
  }
}

class SplayTree<T = SplayPayload> {
  static Node = SplayTreeNode;
  private root_: SplayTreeNode<T> | null = null;

  isEmpty(): boolean {
    return !this.root_;
  }

  insert(key: number, value: T): void {
    if (this.isEmpty()) {
      this.root_ = new SplayTreeNode<T>(key, value);
      return;
    }
    this.splay_(key);
    if (this.root_!.key === key) {
      return;
    }
    const node = new SplayTreeNode<T>(key, value);
    if (key > this.root_!.key) {
      node.left = this.root_;
      node.right = this.root_!.right;
      this.root_!.right = null;
    } else {
      node.right = this.root_;
      node.left = this.root_!.left;
      this.root_!.left = null;
    }
    this.root_ = node;
  }

  remove(key: number): SplayTreeNode<T> {
    if (this.isEmpty()) {
      throw new Error("Key not found: " + key);
    }
    this.splay_(key);
    if (this.root_!.key !== key) {
      throw new Error("Key not found: " + key);
    }
    const removed = this.root_!;
    if (!this.root_!.left) {
      this.root_ = this.root_!.right;
    } else {
      const right = this.root_!.right;
      this.root_ = this.root_!.left;
      this.splay_(key);
      this.root_!.right = right;
    }
    return removed;
  }

  find(key: number): SplayTreeNode<T> | null {
    if (this.isEmpty()) {
      return null;
    }
    this.splay_(key);
    return this.root_!.key === key ? this.root_ : null;
  }

  findMax(startNode?: SplayTreeNode<T> | null): SplayTreeNode<T> | null {
    if (this.isEmpty()) {
      return null;
    }
    let current: SplayTreeNode<T> | null = startNode || this.root_;
    while (current && current.right) {
      current = current.right;
    }
    return current;
  }

  findGreatestLessThan(key: number): SplayTreeNode<T> | null {
    if (this.isEmpty()) {
      return null;
    }
    this.splay_(key);
    if (this.root_!.key < key) {
      return this.root_;
    } else if (this.root_!.left) {
      return this.findMax(this.root_!.left);
    } else {
      return null;
    }
  }

  exportKeys(): number[] {
    const result: number[] = [];
    if (!this.isEmpty()) {
      this.root_!.traverse_((node) => {
        result.push(node.key);
      });
    }
    return result;
  }

  private splay_(key: number): void {
    if (this.isEmpty()) {
      return;
    }
    const dummy = new SplayTreeNode<T>(0, null as unknown as T);
    let left = dummy;
    let right = dummy;
    let current = this.root_!;
    while (true) {
      if (key < current.key) {
        if (!current.left) {
          break;
        }
        if (key < current.left.key) {
          const tmp = current.left;
          current.left = tmp.right;
          tmp.right = current;
          current = tmp;
          if (!current.left) {
            break;
          }
        }
        right.left = current;
        right = current;
        current = current.left;
      } else if (key > current.key) {
        if (!current.right) {
          break;
        }
        if (key > current.right.key) {
          const tmp = current.right;
          current.right = tmp.left;
          tmp.left = current;
          current = tmp;
          if (!current.right) {
            break;
          }
        }
        left.right = current;
        left = current;
        current = current.right;
      } else {
        break;
      }
    }
    left.right = current.left;
    right.left = current.right;
    current.left = dummy.right;
    current.right = dummy.left;
    this.root_ = current;
  }
}

const kSplayTreeSize = 8000;
const kSplayTreeModifications = 80;
const kSplayTreePayloadDepth = 5;

let splayTree: SplayTree<SplayPayload> | null = null;

function GeneratePayloadTree(depth: number, tag: string): SplayPayload {
  if (depth === 0) {
    return {
      array: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
      string: "String for key " + tag + " in leaf node",
    };
  } else {
    return {
      left: GeneratePayloadTree(depth - 1, tag),
      right: GeneratePayloadTree(depth - 1, tag),
    };
  }
}

function GenerateKey(): number {
  return Math.random();
}

function InsertNewNode(): number {
  let key: number;
  do {
    key = GenerateKey();
  } while (splayTree!.find(key) != null);
  const payload = GeneratePayloadTree(kSplayTreePayloadDepth, String(key));
  splayTree!.insert(key, payload);
  return key;
}

function SplaySetup(): void {
  splayTree = new SplayTree<SplayPayload>();
  for (let i = 0; i < kSplayTreeSize; i++) InsertNewNode();
}

function SplayTearDown(): void {
  const keys = splayTree!.exportKeys();
  splayTree = null;
  const length = keys.length;
  if (length !== kSplayTreeSize) {
    throw new Error("Splay tree has wrong size");
  }
  for (let i = 0; i < length - 1; i++) {
    if (keys[i] >= keys[i + 1]) {
      throw new Error("Splay tree not sorted");
    }
  }
}

function SplayRun(): void {
  for (let i = 0; i < kSplayTreeModifications; i++) {
    const key = InsertNewNode();
    const greatest = splayTree!.findGreatestLessThan(key);
    if (greatest == null) splayTree!.remove(key);
    else splayTree!.remove(greatest.key);
  }
}

const Splay = new BenchmarkSuite("Splay", 81491, [
  new Benchmark("Splay", SplayRun, SplaySetup, SplayTearDown),
]);

const globalScope = (typeof globalThis !== "undefined"
  ? globalThis
  : typeof window !== "undefined"
  ? window
  : Function("return this")()) as unknown as Record<string, unknown>;

globalScope.Splay = Splay;
globalScope.SplayTree = SplayTree;
