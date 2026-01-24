export type ApiErrorEnvelope = {
  code?: string;
  message?: string;
};

const TOKEN_KEY = 'fdd-example.access_token';

export function getToken(): string | null {
  try {
    return localStorage.getItem(TOKEN_KEY);
  } catch {
    return null;
  }
}

export function setToken(token: string) {
  try {
    localStorage.setItem(TOKEN_KEY, token);
  } catch {
    // ignore
  }
}

export function clearToken() {
  try {
    localStorage.removeItem(TOKEN_KEY);
  } catch {
    // ignore
  }
}

export class ApiError extends Error {
  public status: number;
  public envelope?: ApiErrorEnvelope;

  constructor(message: string, status: number, envelope?: ApiErrorEnvelope) {
    super(message);
    this.status = status;
    this.envelope = envelope;
  }
}

export async function apiJson<T>(
  path: string,
  options: RequestInit & { token?: string | null } = {},
): Promise<T> {
  const token = options.token ?? getToken();

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-init-request
  const headers = new Headers(options.headers ?? {});
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-init-request

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-if-token
  if (token) {
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-attach-auth-header
    headers.set('authorization', `Bearer ${token}`);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-attach-auth-header
  }
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-if-token

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-if-body
  if (options.body && typeof options.body !== 'string') {
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-serialize-body
    options.body = JSON.stringify(options.body);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-serialize-body
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-attach-body
    headers.set('content-type', 'application/json');
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-attach-body
  }
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-if-body

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-return-prepared
  const res = await fetch(path, {
    ...options,
    headers,
  });
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-return-prepared

  if (res.ok) {
    return (await res.json()) as T;
  }

  let envelope: ApiErrorEnvelope | undefined;
  try {
    envelope = (await res.json()) as ApiErrorEnvelope;
  } catch {
    envelope = undefined;
  }

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-match-status
  const message = envelope?.message ?? `${res.status} ${res.statusText}`;
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-match-status

  throw new ApiError(message, res.status, envelope);
}
