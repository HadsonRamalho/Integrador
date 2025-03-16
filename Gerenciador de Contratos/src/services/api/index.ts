import axios from "axios";

const client = axios.create({
  baseURL: 'https://j1p43lfm-3003.brs.devtunnels.ms',
});

export { client };
