Warning: True color (24-bit) support not detected. Using a terminal with true color enabled will result in a better visual experience.
YOLO mode is enabled. All tool calls will be automatically approved.
YOLO mode is enabled. All tool calls will be automatically approved.
Ripgrep is not available. Falling back to GrepTool.
Attempt 1 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:53:07 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=5597',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': '199340937965891d',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Attempt 2 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:53:18 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=7242',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': '5a62040cde23f288',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Attempt 3 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:53:38 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=7409',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': 'f2c204c629755b5',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Attempt 1 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:55:32 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=7261',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': '42c69a4959d6c2ac',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Attempt 2 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:55:44 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=6237',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': 'ad759c72f5efaeab',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Attempt 3 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:56:02 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=6694',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': '4feb7031d3406f8f',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Attempt 4 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:56:19 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=333',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': 'b00d5f68e96dee42',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Attempt 1 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:58:13 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=7536',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': '74a682792214d010',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Attempt 2 failed with status 429. Retrying with backoff... _GaxiosError: [{
  "error": {
    "code": 429,
    "message": "No capacity available for model gemini-3.1-pro-preview on the server",
    "errors": [
      {
        "message": "No capacity available for model gemini-3.1-pro-preview on the server",
        "domain": "global",
        "reason": "rateLimitExceeded"
      }
    ],
    "status": "RESOURCE_EXHAUSTED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "MODEL_CAPACITY_EXHAUSTED",
        "domain": "cloudcode-pa.googleapis.com",
        "metadata": {
          "model": "gemini-3.1-pro-preview"
        }
      }
    ]
  }
}
]
    at Gaxios._request (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:8811:19)
    at process.processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async _OAuth2Client.requestAsync (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:10774:16)
    at async CodeAssistServer.requestStreamingPost (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272793:17)
    at async CodeAssistServer.generateContentStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:272591:23)
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:273444:19
    at async file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:250345:23
    at async retryWithBackoff (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:270539:23)
    at async GeminiChat.makeApiCallAndProcessStream (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293199:28)
    at async GeminiChat.streamWithRetries (file:///usr/lib/node_modules/@google/gemini-cli/bundle/chunk-6DSAZLFF.js:293037:29) {
  config: {
    url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
    method: 'POST',
    params: { alt: 'sse' },
    headers: {
      'Content-Type': 'application/json',
      'User-Agent': 'GeminiCLI/0.41.2/gemini-3.1-pro-preview (linux; x64; terminal) google-api-nodejs-client/9.15.1',
      Authorization: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      'x-goog-api-client': 'gl-node/22.22.0'
    },
    responseType: 'stream',
    body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
    signal: AbortSignal { aborted: false },
    retry: false,
    paramsSerializer: [Function: paramsSerializer],
    validateStatus: [Function: validateStatus],
    errorRedactor: [Function: defaultErrorRedactor]
  },
  response: {
    config: {
      url: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse',
      method: 'POST',
      params: [Object],
      headers: [Object],
      responseType: 'stream',
      body: '<<REDACTED> - See `errorRedactor` option in `gaxios` for configuration>.',
      signal: [AbortSignal],
      retry: false,
      paramsSerializer: [Function: paramsSerializer],
      validateStatus: [Function: validateStatus],
      errorRedactor: [Function: defaultErrorRedactor]
    },
    data: '[{\n' +
      '  "error": {\n' +
      '    "code": 429,\n' +
      '    "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '    "errors": [\n' +
      '      {\n' +
      '        "message": "No capacity available for model gemini-3.1-pro-preview on the server",\n' +
      '        "domain": "global",\n' +
      '        "reason": "rateLimitExceeded"\n' +
      '      }\n' +
      '    ],\n' +
      '    "status": "RESOURCE_EXHAUSTED",\n' +
      '    "details": [\n' +
      '      {\n' +
      '        "@type": "type.googleapis.com/google.rpc.ErrorInfo",\n' +
      '        "reason": "MODEL_CAPACITY_EXHAUSTED",\n' +
      '        "domain": "cloudcode-pa.googleapis.com",\n' +
      '        "metadata": {\n' +
      '          "model": "gemini-3.1-pro-preview"\n' +
      '        }\n' +
      '      }\n' +
      '    ]\n' +
      '  }\n' +
      '}\n' +
      ']',
    headers: {
      'content-length': '630',
      'content-type': 'application/json; charset=UTF-8',
      date: 'Mon, 11 May 2026 12:58:17 GMT',
      server: 'ESF',
      'server-timing': 'gfet4t7; dur=322',
      vary: 'Origin, X-Origin, Referer',
      'x-cloudaicompanion-trace-id': 'aa27935ba2b68bd6',
      'x-content-type-options': 'nosniff',
      'x-frame-options': 'SAMEORIGIN',
      'x-xss-protection': '0'
    },
    status: 429,
    statusText: 'Too Many Requests',
    request: {
      responseURL: 'https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse'
    }
  },
  error: undefined,
  status: 429,
  [Symbol(gaxios-gaxios-error)]: '6.7.1'
}
Q1: PASS — `resume_existing_durable` correctly inverts the existence check, parses the manifest, and loads secrets from the durable keystore.
Q2: PASS — `ManifestAbsentInResume` variant itself is explicitly fail-closed.
Q3: VETO — `chain_runtime.rs` env gate drifts from kernel-side by appending `&& ...exists()`, causing a silent fallback to `generate_or_load_durable` (fresh init) when resuming with a missing manifest, rendering `ManifestAbsentInResume` unreachable.
Q4: PASS — Fresh-path behavior correctly calls the unchanged `generate_or_load_durable` path.
Q5: PASS — `sg_g1_5_pinned_pubkeys_preserved_across_resume` passes, preserving kernel-side G1.1 logic without regression.
Q6: PASS — Hash `a2d0f3bfa50bd7f45e3f2c8fe51add525127d30d3f4e61302f9306916b95d44d` matches the Trust Root manifest entry exactly.
Q7: PASS — Verified via source-grep; no f64, shadow ledger, or admission-arm surface introduced.
Q8: PASS — `agent_pubkeys.json` is safely preserved on successful resume, aligning with FC2 §3.2 semantics.
Q9: PASS — `ManifestAbsentInResume` provides explicit, Drucker-compliant guidance referencing FC2 §3.2.

Aggregate R1.5 verdict: VETO
Conviction: high
Recommendation: R2
