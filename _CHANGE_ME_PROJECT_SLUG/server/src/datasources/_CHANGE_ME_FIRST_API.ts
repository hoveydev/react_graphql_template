// TODO: change file name
import { RESTDataSource } from "@apollo/datasource-rest";
import { _CHANGE_ME_FIRST_TYPE } from '../types'

// TODO: remove this
const mockData: _CHANGE_ME_FIRST_TYPE = {
  fieldOne: 'data for field one',
  fieldTwo: 'data for field two'
}

export class _CHANGE_ME_FIRST_API extends RESTDataSource {
  baseURL = '_CHANGE_ME_REST_URL';

  getData() {
    // return this.get<FirstType>('all-data');
    return mockData; // TODO: replace with real data
  }
}
