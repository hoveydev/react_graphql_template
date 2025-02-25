import { ApolloServer } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';
import typeDefs from './schemas';
import resolvers from './resolvers';
// TODO: Import correct classname
import { _CHANGE_ME_FIRST_API } from './datasources/api';

async function startApolloServer() {
  const server = new ApolloServer({ typeDefs, resolvers });
  const { url } = await startStandaloneServer(server, {
    context: async () => {
      const { cache } = server;
      return {
        dataSources: {
          // TODO: Be sure to run `npm generate` before changing below
          _CHANGE_ME_FIRST_API_DATASOURCE: new _CHANGE_ME_FIRST_API({ cache }),
        },
      };
    },
  });
  console.log(`🚀 Server listening at: ${url}`);
}

startApolloServer();
