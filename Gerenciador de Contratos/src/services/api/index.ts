import axios from "axios";

const client = axios.create({
  baseURL: import.meta.env.BASE_URL,
});

export { client };
