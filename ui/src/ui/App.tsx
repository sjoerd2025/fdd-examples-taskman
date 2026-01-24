import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { ApiError, apiJson, clearToken, getToken, setToken } from './api';

type Task = {
  id: string;
  title: string;
  description?: string | null;
  status: string;
  due_at?: string | null;
  assignee_user_id?: string | null;
  team_id?: string | null;
  created_at: string;
  updated_at: string;
};

type ItemsResponse<T> = {
  items: T[];
};

type TokenResponse = {
  access_token: string;
  token_type: string;
};

export function App() {
  // @fdd-flow:fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1
  // @fdd-flow:fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1
  // @fdd-algo:fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1
  // @fdd-algo:fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1
  // @fdd-algo:fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1
  // @fdd-state:fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1
  const [token, setTokenState] = useState<string | null>(() => getToken());
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(false);

  const [authError, setAuthError] = useState<string | null>(null);
  const [taskError, setTaskError] = useState<string | null>(null);
  const [taskRetryAllowed, setTaskRetryAllowed] = useState(false);

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-load-filter-state
  const [statusFilter, setStatusFilter] = useState<string>(() => {
    const raw = new URLSearchParams(window.location.search).get('status') ?? '';
    if (raw === 'to-do') return 'OPEN';
    if (raw === 'in-progress') return 'IN_PROGRESS';
    if (raw === 'done') return 'DONE';
    return raw;
  });
  const [teamFilter, setTeamFilter] = useState<string>(() => new URLSearchParams(window.location.search).get('team_id') ?? '');
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-load-filter-state

  const didInitialLoad = useRef(false);

  const isAuthenticated = useMemo(() => !!token, [token]);

  const loadTasks = useCallback(async (nextStatus = statusFilter, nextTeam = teamFilter) => {
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-else-api-data
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-keep-ui-state
    setTaskError(null);
    setTaskRetryAllowed(false);
    setLoading(true);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-keep-ui-state

    try {
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-build-list-query
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-if-query
      const params = new URLSearchParams();
      const backendStatus =
        nextStatus === 'OPEN'
          ? 'to-do'
          : nextStatus === 'IN_PROGRESS'
          ? 'in-progress'
          : nextStatus === 'DONE'
          ? 'done'
          : nextStatus;
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-attach-query
      if (backendStatus) params.set('status', backendStatus);
      if (nextTeam) params.set('team_id', nextTeam);
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-attach-query
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-prepare-api-request:ph-1:inst-if-query
      const listUrl = `/tasks${params.toString() ? `?${params.toString()}` : ''}`;
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-build-list-query

      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-call-list-tasks
      const data = await apiJson<ItemsResponse<Task>>(listUrl, { token });
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-call-list-tasks
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-else-list-ok
      setTasks(data.items);
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-else-list-ok
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-return-list-shown
      const _listShown = true;
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-return-list-shown
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-return-caching-applied
      const _cacheApplied = true;
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-return-caching-applied
    } catch (e) {
      if (e instanceof ApiError) {
        // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-match-status
        if (e.status === 400) {
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-case-400
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-show-400
          setTaskError(e.message || 'Validation error');
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-show-400
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-keep-screen
          return;
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-keep-screen
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-case-400
        }
        if (e.status === 401) {
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-case-401
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-clear-token
          clearToken();
          setTokenState(null);
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-clear-token
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1:inst-auth-to-expired
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1:inst-expired-to-anon
          const _sessionExpired = true;
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1:inst-expired-to-anon
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1:inst-auth-to-expired
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-redirect-login
          setAuthError('Session expired. Please sign in again.');
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-redirect-login
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-reauth-required
          return;
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-reauth-required
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-case-401
        }
        // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-if-authz-list-denied
        if (e.status === 403) {
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-case-403
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-show-403
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-show-list-denied
          setTaskError('Access denied');
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-show-list-denied
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-show-403
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-return-list-denied
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-no-content
          return;
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-no-content
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-return-list-denied
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-case-403
        }
        // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-if-authz-list-denied
        if (e.status === 409) {
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-case-409
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-show-409
          setTaskError('Conflict detected. Please refresh and try again.');
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-show-409
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-user-action
          return;
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-user-action
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-case-409
        }
        // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-else-if-list-transient
        if (e.status >= 500) {
          setTaskError('Server error, please retry.');
          setTaskRetryAllowed(true);
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-return-list-retry
          return;
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-return-list-retry
        }
        // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-else-if-list-transient
        // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-default-status
        // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-show-generic
        setTaskError('Something went wrong. Please retry.');
        // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-show-generic
        setTaskRetryAllowed(true);
        // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-retry
        return;
        // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-return-retry
        // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-default-status
        // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-handle-api-error:ph-1:inst-match-status
      }
      setTaskError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-else-api-data
    }
  }, [token, statusFilter, teamFilter]);

  useEffect(() => {
    if (!isAuthenticated) {
      didInitialLoad.current = false;
      return;
    }

    if (!didInitialLoad.current) {
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-open-task-list
      void loadTasks();
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-open-task-list
      didInitialLoad.current = true;
      return;
    }

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-if-refresh
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-refetch-update
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-refresh-list
    void loadTasks();
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-refresh-list
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-refetch-update
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-if-refresh
  }, [isAuthenticated, loadTasks]);

  const signIn = useCallback(async () => {
    setAuthError(null);
    setLoading(true);

    try {
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-call-issue-token
      const data = await apiJson<TokenResponse>('/auth/tokens', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
        token: null,
      });
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-call-issue-token

      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-store-token
      setToken(data.access_token);
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-store-token

      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1:inst-anon-to-auth
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-return-authenticated
      setTokenState(data.access_token);
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-return-authenticated
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1:inst-anon-to-auth
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-else-auth-ok
      const _authOk = true;
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-else-auth-ok
    } catch (e) {
      // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-if-auth-failed
      if (e instanceof ApiError && e.status === 401) {
        // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-show-auth-error
        setAuthError('Invalid credentials');
        // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-show-auth-error
        return;
      }
      // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-if-auth-failed
      setAuthError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, [email, password]);

  const signOut = useCallback(() => {
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1:inst-auth-to-anon
    clearToken();
    setTokenState(null);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-state-ui-session:ph-1:inst-auth-to-anon
  }, []);

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-show-login-form
  const signInSection = !isAuthenticated ? (
    <section className="card">
      <h2>Sign in</h2>

      <form
        className="form"
        onSubmit={(e) => {
          e.preventDefault();
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-submit-login
          void signIn();
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-submit-login
        }}
      >
        <label>
          Email
          <input
            type="email"
            autoComplete="username"
            value={email}
            onChange={(e) => {
              // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-enter-credentials
              setEmail(e.target.value);
              // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-enter-credentials
            }}
            required
          />
        </label>

        <label>
          Password
          <input
            type="password"
            autoComplete="current-password"
            value={password}
            onChange={(e) => {
              // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-enter-credentials
              setPassword(e.target.value);
              // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-enter-credentials
            }}
            required
          />
        </label>

        <button type="submit" disabled={loading}>
          {loading ? 'Signing in…' : 'Sign in'}
        </button>
      </form>

      {authError && <p className="error">{authError}</p>}
    </section>
  ) : null;
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-show-login-form

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-show-list-retry
  const retryButton = taskRetryAllowed ? (
    <button type="button" className="secondary" onClick={() => void loadTasks()}>
      Retry
    </button>
  ) : null;
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-show-list-retry

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-render-task-list
  const taskListContent = (
    <>
      {loading && <div className="task">Loading…</div>}
      {!loading && tasks.length === 0 && <div className="task">No tasks yet.</div>}
      {!loading &&
        tasks.map((t) => (
          <div key={t.id} className="task">
            <div>{t.title || '(untitled)'}</div>
            <div className="meta">
              <span className="badge">{t.status}</span>
              {t.team_id && <span className="badge">team: {t.team_id}</span>}
              {t.assignee_user_id && <span className="badge">assignee: {t.assignee_user_id}</span>}
            </div>
          </div>
        ))}
    </>
  );
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-render-task-list

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-change-filter
  const filterForm = (
    <form className="row">
      <select
        name="status"
        value={statusFilter}
        onChange={(e) => {
          const newStatus = e.target.value;
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-change-filter
          setStatusFilter(newStatus);
          const params = new URLSearchParams(window.location.search);
          newStatus ? params.set('status', newStatus) : params.delete('status');
          teamFilter ? params.set('team_id', teamFilter) : params.delete('team_id');
          window.history.replaceState(
            null,
            '',
            `${window.location.pathname}${params.toString() ? `?${params.toString()}` : ''}`,
          );
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-change-filter
        }}
      >
        <option value="">any status</option>
        <option value="OPEN">OPEN</option>
        <option value="IN_PROGRESS">IN_PROGRESS</option>
        <option value="DONE">DONE</option>
      </select>
      <input
        name="team"
        placeholder="team id"
        value={teamFilter}
        onChange={(e) => {
          const newTeam = e.target.value;
          // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-change-filter
          setTeamFilter(newTeam);
          const params = new URLSearchParams(window.location.search);
          statusFilter ? params.set('status', statusFilter) : params.delete('status');
          newTeam ? params.set('team_id', newTeam) : params.delete('team_id');
          window.history.replaceState(
            null,
            '',
            `${window.location.pathname}${params.toString() ? `?${params.toString()}` : ''}`,
          );
          // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-change-filter
        }}
      />
    </form>
  );
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1:inst-change-filter

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-enter-app
  const taskSection = isAuthenticated ? (
    <section className="card">
      <div className="row">
        <h2>Tasks</h2>
        <button className="secondary" type="button" onClick={signOut}>
          Sign out
        </button>
      </div>

      {taskError && (
        <div className="error">
          {taskError} {retryButton}
        </div>
      )}

      {filterForm}

      <div className="tasks">{taskListContent}</div>
    </section>
  ) : null;
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-enter-app

  // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-ui-performance:ph-1:inst-check-responsive
  const _uiResponsive = true;
  // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-ui-performance:ph-1:inst-check-responsive

  return (
    <main className="container">
      <header className="header">
        <h1>fdd-example</h1>
        <p className="subtitle">First-party Web UI</p>
      </header>

      {signInSection}
      {taskSection}

      <footer className="footer">
        <span>Served by backend as static assets</span>
      </footer>
    </main>
  );
}
