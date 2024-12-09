export const UserMenuFallback = ({ error }) => {
  console.error('UserMenuFallback:', error.message);
  return (
    // will have to get more info how to handle UserMenu from the team
    <div>
      <h1>UserMenuFallback</h1>
    </div>
  );
};
