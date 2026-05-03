import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
  vus: 10,
  duration: '6m',
};

export default function () {
  http.get('http://localhost:3000');
  sleep(1);

  http.get('http://localhost:3000/say-hello/John');
  sleep(1);

  // Run this to test the error path, randomly
  if (Math.random() < 0.5) {
    http.get('http://localhost:3000/oops');
    sleep(1);
  }
}
