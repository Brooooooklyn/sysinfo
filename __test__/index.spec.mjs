import test from 'ava'

import { cpuFeatures } from '../index.js'

test('cpuFeatures', (t) => {
  t.notThrows(() => cpuFeatures())
})
