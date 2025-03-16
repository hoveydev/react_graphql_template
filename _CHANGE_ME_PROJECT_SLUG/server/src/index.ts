import { ApolloServer } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';

import { _CHANGE_ME_FIRST_API } from './datasources/_CHANGE_ME_FIRST_API';
import resolvers from './resolvers';
import typeDefs from './schemas';
// TODO: Import correct classname

const startApolloServer = async () => {
  const server = new ApolloServer({ typeDefs, resolvers });
  const { url } = await startStandaloneServer(server, {
    context: async () => {
      const { cache } = server;
      return {
        dataSources: {
          // TODO: Be sure to run `npm generate` before changing below
          _CHANGE_ME_FIRST_API_DATASOURCE: new _CHANGE_ME_FIRST_API({ cache })
        }
      };
    },
    listen: { port: parseInt(process.env.PORT) || 4000 }
  });
  // eslint-disable-next-line no-console
  console.log(`🚀 Server listening at: ${url}`);
};

startApolloServer();
