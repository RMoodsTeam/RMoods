import Demo from '../components/Demo';
import { Center, Container, Title } from '@mantine/core';

const Root = () => {
  return (
    <Container>
      <Center>
        <Title order={1}>RMoods Demo Content</Title>
      </Center>
      <Demo />
    </Container>
  );
};

export default Root;
