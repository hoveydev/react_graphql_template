import js from "@eslint/js";
import globals from "globals";
import reactEslintPlugin from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import eslintPrettier from "eslint-plugin-prettier/recommended";
import importOrder from "eslint-plugin-import";
import tseslint from "typescript-eslint";

export default tseslint.config(
  {
    ignores: [
      "client/dist",
      "client/eslint.config.js",
      "server/build",
      "server/codegen.ts",
      "server/src/types.ts",
    ],
  },
  {
    extends: [
      js.configs.recommended,
      ...tseslint.configs.recommended,
      eslintPrettier,
    ],
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      ecmaVersion: 2020,
      globals: { ...globals.browser, ...globals.node },
    },
    rules: {
      quotes: ["error", "single"],
      semi: ["error", "always"],
      "no-unused-vars": "error",
      "arrow-body-style": ["error", "always"],
      "default-case": "error",
      "default-case-last": "warn",
      "dot-notation": "warn",
      "no-caller": "error",
      "no-console": "warn",
      "no-eval": "error",
      "no-labels": "error",
      "no-octal-escape": "error",
      "no-param-reassign": "error",
      "no-promise-executor-return": "error",
      "no-restricted-syntax": [
        "error",
        {
          selector: "SequenceExpression",
          message:
            "The comma operator is confusing and a common mistake. Don’t use it!",
        },
      ],
      "no-self-compare": "error",
      "no-shadow": "error",
      "no-template-curly-in-string": "error",
      "no-unmodified-loop-condition": "error",
      "no-unneeded-ternary": "warn",
      "no-useless-backreference": "error",
      "no-useless-computed-key": "warn",
      "no-useless-concat": "warn",
      "no-useless-constructor": "warn",
      "no-useless-rename": "warn",
      "no-var": "warn",
      "object-shorthand": "warn",
      "one-var": ["warn", "never"],
      "prefer-arrow-callback": "warn",
      "prefer-const": "warn",
      "prefer-destructuring": ["warn", { object: true, array: false }],
      "prefer-exponentiation-operator": "warn",
      "prefer-numeric-literals": "warn",
      "prefer-object-spread": "warn",
      "prefer-promise-reject-errors": "error",
      "prefer-regex-literals": "warn",
      "prefer-rest-params": "warn",
      "prefer-spread": "warn",
      "prefer-template": "warn",
      curly: "warn",
      eqeqeq: ["error", "always", { null: "ignore" }],
      strict: "error",
      yoda: "warn",
      "func-style": ["error", "expression"],
      "prettier/prettier": [
        "error",
        {
          singleQuote: true,
          trailingComma: "none",
        },
      ],
    },
  },
  // Client specific rules
  {
    files: ["client/**/*"],
    plugins: {
      react: reactEslintPlugin,
      import: importOrder,
      "react-hooks": reactHooks,
      "react-refresh": reactRefresh,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true },
      ],
      "import/order": [
        "error",
        {
          groups: ["builtin", "external", "internal"],
          pathGroups: [
            {
              pattern: "react",
              group: "external",
              position: "before",
            },
          ],
          pathGroupsExcludedImportTypes: ["react"],
          "newlines-between": "always",
          alphabetize: {
            order: "asc",
            caseInsensitive: true,
          },
        },
      ],
    },
  },
  // Server specific rules
  {
    files: ["server/**/*"],
    plugins: {
      import: importOrder,
    },
    rules: {
      "import/order": [
        "error",
        {
          groups: ["builtin", "external", "internal"],
          "newlines-between": "always",
          alphabetize: {
            order: "asc",
            caseInsensitive: true,
          },
        },
      ],
    },
  },
);
