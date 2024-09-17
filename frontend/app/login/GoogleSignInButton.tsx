import React from "react";
import { Button } from "@mui/joy";
import Image from "next/image";

/**
 * @param params - onClick function
 * @returns Element
 */
export default function GoogleSignInButton(params: { onClick: any }) {
  return (
    <Button
      variant="outlined"
      onClick={params.onClick}
      id="google-sign-in-button"
      startDecorator={
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
