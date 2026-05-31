import http from 'k6/http';
import { check, sleep } from 'k6';

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

// Two parallel scenarios:
//   user_flow   — JWT-authenticated reads (simulates web/mobile clients)
//   device_flow — API-key-authenticated writes (simulates IoT sensors)
export const options = {
  scenarios: {
    user_flow: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '20s', target: 30 }, // ramp up
        { duration: '60s', target: 30 }, // steady state
        { duration: '10s', target: 0  }, // ramp down
      ],
      exec: 'userScenario',
    },
    device_flow: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '20s', target: 20 },
        { duration: '60s', target: 20 },
        { duration: '10s', target: 0  },
      ],
      exec: 'deviceScenario',
    },
  },

  thresholds: {
    'http_req_duration{scenario:user_flow}':   ['p(95)<1000'],
    'http_req_duration{scenario:device_flow}': ['p(95)<500'],
    http_req_failed: ['rate<0.05'],
  },
};

// Runs once before all VUs start. Creates a test user, reservoir, and device.
// The returned object is passed as `data` to each VU function and teardown.
export function setup() {
  const headers = { 'Content-Type': 'application/json' };
  const email = `loadtest_${Date.now()}@example.com`;

  const registerRes = http.post(
    `${BASE_URL}/auth/register`,
    JSON.stringify({
      email,
      password: 'LoadTest123!',
      first_name: 'Load',
      last_name: 'Test',
    }),
    { headers }
  );

  check(registerRes, { 'setup: registered': (r) => r.status === 201 });
  if (registerRes.status !== 201) {
    throw new Error(`Registration failed (${registerRes.status}): ${registerRes.body}`);
  }

  const token = registerRes.json('token');
  const authHeaders = {
    'Content-Type': 'application/json',
    Authorization: `Bearer ${token}`,
  };

  const reservoirRes = http.post(
    `${BASE_URL}/reservoirs`,
    JSON.stringify({ name: 'Load Test Tank', capacity: 5000.0, location: 'Lab' }),
    { headers: authHeaders }
  );
  check(reservoirRes, { 'setup: reservoir created': (r) => r.status === 201 });
  const reservoirId = reservoirRes.json('id');

  const deviceRes = http.post(
    `${BASE_URL}/devices`,
    JSON.stringify({ name: 'Load Test Sensor', reservoir_id: reservoirId }),
    { headers: authHeaders }
  );
  check(deviceRes, { 'setup: device created': (r) => r.status === 201 });

  return {
    token,
    reservoirId,
    deviceId: deviceRes.json('id'),
    // api_key is returned in full only on device creation
    apiKey: deviceRes.json('api_key'),
  };
}

// JWT scenario: list reservoirs → list devices → fetch measurement history
export function userScenario(data) {
  const headers = { Authorization: `Bearer ${data.token}` };

  const r1 = http.get(`${BASE_URL}/reservoirs`, { headers });
  check(r1, { 'GET /reservoirs 200': (r) => r.status === 200 });

  const r2 = http.get(`${BASE_URL}/devices`, { headers });
  check(r2, { 'GET /devices 200': (r) => r.status === 200 });

  const r3 = http.get(
    `${BASE_URL}/devices/${data.deviceId}/measurements?limit=50`,
    { headers }
  );
  check(r3, { 'GET /measurements 200': (r) => r.status === 200 });

  sleep(0.5);
}

// API-key scenario: submit a sensor measurement
export function deviceScenario(data) {
  const res = http.post(
    `${BASE_URL}/devices/measurements`,
    JSON.stringify([{ value: Math.random() * 1000 }]),
    {
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': data.apiKey,
      },
    }
  );
  check(res, { 'POST /measurements 201': (r) => r.status === 201 });

  sleep(0.1);
}

// Runs once after all VUs finish. Deletes the reservoir (cascades to device + measurements).
export function teardown(data) {
  http.del(
    `${BASE_URL}/reservoirs/${data.reservoirId}`,
    null,
    {
      headers: {
        Authorization: `Bearer ${data.token}`,
      },
    }
  );
}
