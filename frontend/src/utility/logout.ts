import { NavigateFunction } from 'react-router-dom';
import Cookies from 'js-cookie';

export const logout = (navigate: NavigateFunction) => {
  Cookies.remove('RMOODS_JWT');
  navigate('/login');
};
