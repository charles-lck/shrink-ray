import axios from 'axios';

const instance = axios.create({
  timeout: 60000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 请求拦截器
instance.interceptors.request.use(
  (config) => {
    // config.headers.Authorization = `Bearer ${token}`;
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);


instance.interceptors.response.use(
  (response) => {
    return response.data;
  },
  (error) => {
    return Promise.reject(error)
  }
);


export const get = (url: string, params = {}) => {
  return instance
    .get(url, { params })
    .then((response) => response)
    .catch((error) => {
      console.error(error);
      throw error;
    });
};

export const post = (url: string, data = {}) => {
  return instance
    .post(url, data)
    .then((response) => response)
    .catch((error) => {
      console.error(error);
      throw error;
    });
};

export const put = (url: string, data = {}) => {
  return instance
    .put(url, data)
    .then((response) => response)
    .catch((error) => {
      console.error(error);
      throw error;
    });
};

export const del = (url: string) => {
  return instance
    .delete(url)
    .then((response) => response)
    .catch((error) => {
      console.error(error);
      throw error;
    });
};
