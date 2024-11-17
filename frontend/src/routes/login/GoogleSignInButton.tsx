import {
  Button,
  Image,
  Box,
} from "@mantine/core";

export default function GoogleSignInButton({ onClick }: { onClick: () => void }) {
  return (
    
          <Button
            variant="default"
            onClick={onClick}
            size="lg"
            radius="xl"
            leftSection={
              <Box
                style={{
                  width: 24,
                  height: 24,
                }}
              >
                <Image
                  src="https://www.svgrepo.com/show/303108/google-icon-logo.svg"
                  alt="google logo"
                />
              </Box>
            }
          >
            Google
          </Button>
        
  );
}
