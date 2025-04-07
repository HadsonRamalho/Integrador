import axios from "axios";

const client = axios.create({
  baseURL: 'http://localhost:3003',
});

export { client };
