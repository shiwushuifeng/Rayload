/** @fileoverview Thin entry point; demo builds install the fake backend before boot. */
import { DEMO_MODE } from './demo/mode'

if (DEMO_MODE) {
  void import('./demo/fake').then(({ installDemoBackend }) => {
    installDemoBackend()
    return import('./main-app')
  })
} else {
  void import('./main-app')
}
