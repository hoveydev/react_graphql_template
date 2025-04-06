import { ApolloServer } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';

import { _CHANGE_ME_FIRST_API_CLASS } from './datasources/_CHANGE_ME_FIRST_API_FILE';
import resolvers from './resolvers';
import typeDefs from './schemas';

const startApolloServer = async () => {
  const server = new ApolloServer({ typeDefs, resolvers });
  const { url } = await startStandaloneServer(server, {
    context: async () => {
      const { cache } = server;
      return {
        dataSources: {
          _CHANGE_ME_FIRST_API_DATASOURCE: new _CHANGE_ME_FIRST_API_CLASS({
            cache
          })
        }
      };
    },
    listen: { port: parseInt(process.env.PORT) || 4000 }
  });
  // eslint-disable-next-line no-console
  console.log(`🚀 Server listening at: ${url}`);
};

startApolloServer();
