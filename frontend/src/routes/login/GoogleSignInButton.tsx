import { Button, Image } from '@mantine/core';

/**
 * @param params - onClick function
 * @returns Element
 */
export default function GoogleSignInButton(params: { onClick: () => void }) {
  return (
    <Button
      variant="outline"
      onClick={params.onClick}
      id="google-sign-in-button"
      leftSection={
        <Image
          src="https://www.svgrepo.com/show/475656/google-color.svg"
          loading="lazy"
          alt="google logo"
          width={30}
          height={30}
        />
      }
    >
      <div id="google-sign-in-button-text">Continue with Google</div>
    </Button>
  );
}
