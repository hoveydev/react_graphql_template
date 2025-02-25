import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: './src/schemas/**/*.gql',
  generates: {
    './src/types.ts': {
      plugins: ['typescript', 'typescript-resolvers'],
      config: {
        contextType: './context#DataSourceContext',
        namingConvention: 'keep',
      },
    },
  },
};

export default config;
