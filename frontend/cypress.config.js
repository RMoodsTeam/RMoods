import { defineConfig } from 'cypress';

export default defineConfig({
  component: {
    devServer: {
      framework: 'react',
      bundler: 'vite',
    },
  },

  e2e: {
    specPattern:
      process.env.TEST_TYPE === 'visual'
        ? 'cypress/visual/**/*.cy.ts'
        : 'cypress/e2e/**/*.cy.ts',
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
  },
});
