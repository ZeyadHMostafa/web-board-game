import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
  globalIgnores([
    'dist',
    'node_modules',
    '**/*.md',
    '**/*.json',
    '**/*.yml',
    '**/*.yaml'
  ]),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      globals: globals.browser,
    },
  },
  {
    files: ['**/*.{ts,tsx}'],
    plugins: {
      'ai-comment-blocker': {
        rules: {
          'no-ai-comments': {
            create(context) {
              return {
                Program() {
                  const sourceCode = context.sourceCode;
                  const comments = sourceCode.getAllComments();

                  // Regex patterns
                  const emojiRegex = /[\u{1F300}-\u{1F9FF}\u{2600}-\u{26FF}\u{2700}-\u{27BF}\u{1F600}-\u{1F64F}\u{1F680}-\u{1F6FF}\u{1F900}-\u{1F9FF}]/u;
                  const wordRegex = /\b(fix|change|add|you|your|yours|you're)\b/i;

                  comments.forEach((comment) => {
                    // Check for Emojis
                    if (emojiRegex.test(comment.value)) {
                      context.report({
                        loc: comment.loc,
                        message: 'AI-style formatting detected: Emojis are completely banned in comments.',
                      });
                    }

                    // Check for Banned Words/Prefixes
                    if (wordRegex.test(comment.value)) {
                      context.report({
                        loc: comment.loc,
                        message: 'AI comment signature detected: Do not use action-prefixes (fix/change/add) or second person pronouns (you/your).',
                      });
                    }
                  });
                },
              };
            },
          },
        },
      },
    },
    rules: {
      'ai-comment-blocker/no-ai-comments': 'error',
      'tailwindcss/no-custom-classname': ['error'], 
      'tailwindcss/no-arbitrary-value': ['error'], 
    },
  },
])
