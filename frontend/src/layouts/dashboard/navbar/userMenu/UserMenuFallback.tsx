import { logout } from '../../../../utility/logout.ts';
import { useNavigate } from 'react-router-dom';

export const UserMenuFallback = ({ error, resetErrorBoundary }) => {
  resetErrorBoundary();
  console.error('UserMenuFallback:', error.message);
  return (
    // TODO: will have to get more info how to handle UserMenu from the team
    <></>
  );
};
