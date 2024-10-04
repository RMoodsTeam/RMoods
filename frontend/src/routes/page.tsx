import { Center, Container, Heading } from "@chakra-ui/react";
import Demo from "../components/Demo";

const Root = () => {
  return (
    <Container>
      <Center>
        <Heading as={"h1"}>RMoods Demo Content</Heading>
      </Center>
      <Demo />
    </Container>
  );
};

export default Root;
