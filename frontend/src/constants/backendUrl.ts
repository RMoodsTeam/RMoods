const BACKEND_URL: string = import.meta.env.PROD
  ? //this will be changed when the backend server will be deployed
    'https://localhost:8001'
  : 'http://localhost:8001';

export default BACKEND_URL;
