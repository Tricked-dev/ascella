import {
  Box,
  Button,
  FormControl,
  FormHelperText,
  Input,
  InputGroup,
  InputRightElement,
  Link,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  Spinner,
  Text,
  useDisclosure,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useState } from "react";
import withSession from "../utils/withSession";
export default function Index() {
  const [spinner, setSpinner] = useState(false);
  const [show, setShow] = useState(false);
  const handleClick = () => setShow(!show);
  const { isOpen, onOpen, onClose } = useDisclosure();

  const Router = useRouter();
  let logout = async (event: any) => {
    event.preventDefault();
    setSpinner(true);
    let r = await fetch("/api/logout", {
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
    });
    if (r.ok) {
      Router.push(
        {
          pathname: "/",
        },
        undefined,
        { shallow: true },
      );
    } else {
      setSpinner(false);
      onOpen();
    }
  };

  return (
    <>
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Failed to logout</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            Its likely that you don&apos;t have a account you can obtain one in the discord{" "}
            <Link color="blue.400" href="https://discord.gg/KkMKCchJb8">
              https://discord.gg/KkMKCchJb8
            </Link>
          </ModalBody>

          <ModalFooter>
            <Button colorScheme="blue" mr={3} onClick={onClose}>
              Close
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
      <Box height="4rem"></Box>
      <Box
        top="10rem"
        alignContent={"center"}
        justifyItems={"center"}
        maxW={"50rem"}
        margin={"auto"}
        backgroundColor={"gray.900"}
        borderRadius={"10rem"}
        padding="1.5rem"
        borderColor={"gray.900"}
        rounded="md"
        alignItems={"center"}
      >
        <form onSubmit={logout}>
          <Box alignContent={"center"} display="flex" justifyContent={"center"}>
            {spinner && (
              <Spinner
                thickness="4px"
                speed="0.65s"
                emptyColor="gray.200"
                color="blue.500"
                size="xl"
              />
            )}
            {!spinner && (
              <Button type="submit" variant="solid">
                logout
              </Button>
            )}
          </Box>
        </form>
      </Box>
    </>
  );
}
export const getServerSideProps = withSession(async function({
  req,
  res,
}: any) {
  return {
    props: {},
  };
});
