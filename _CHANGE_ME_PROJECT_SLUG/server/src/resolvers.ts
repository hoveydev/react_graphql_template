import { Resolvers } from "./types";

export const resolvers: Resolvers = {
  Query: {
    type: (_, __, { dataSources }) => {
      // TODO: Be sure to run `npm generate` before changing below
      return dataSources._CHANGE_ME_FIRST_API_DATASOURCE.getData();
    },
  }
};

export default resolvers;
