import {
  Button,
  Image,
  Text,
  Paper,
  Stack,
  Center,
  Title,
  Box,
} from "@mantine/core";

export default function LoginPage({ onClick }: { onClick: () => void }) {
  return (
    <Center p="xl">
      <Paper radius="md" w="540" p="xl" withBorder shadow="md">
        <Stack gap="lg" align="center">
          <div>
            <Title order={1} ta="center">
              Welcome to RMoods!
            </Title>
            <Text c="dimmed" size="lg" ta="center" mt="sm">
              Sign in to continue
            </Text>
          </div>

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
        </Stack>
      </Paper>
    </Center>
  );
}
