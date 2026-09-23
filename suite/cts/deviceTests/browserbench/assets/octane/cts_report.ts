/*
 * Copyright (C) 2026 The AOSP and FrizkOS
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 */

export {};

declare global {
  interface Window {
    CtsReport: (message: string, score: number | string, isFinal: boolean) => void;
  }
}

/** Reports a benchmark result to the CTS test server. */
function CtsReport(message: string, score: number | string, isFinal: boolean): void {
  const request = new XMLHttpRequest();
  const query = new URLSearchParams({
    final: isFinal ? '1' : '0',
    score: String(score),
    message,
  });

  request.open('POST', `cts_report.html?${query.toString()}`, false);
  request.send(null);
}

window.CtsReport = CtsReport;
