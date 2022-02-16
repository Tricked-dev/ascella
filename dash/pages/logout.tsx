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
            <Link color="blue.400" href="https://discord.gg/mY8zTARu4g">
              https://discord.gg/mY8zTARu4g
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

// import Head from 'next/head';
// import { GetServerSideProps } from 'next';
// import { DiscordUser } from '../utils/types';
// import { parseUser } from '../utils/parse-user';
// import { Box, Image, Text, Wrap, Link } from '@chakra-ui/react';
// import axios from 'axios';
// interface Props {
// 	user: DiscordUser;
// }

// export default function Home({ user, guilds }: any) {
// 	return (
// 		<div className="flex flex-col items-center justify-center min-h-screen py-3">
// 			<Text variant="h2" fontSize="2xl">
// 				Choose a server to view the suggestions of
// 			</Text>
// 			<Wrap>
// 				{guilds.map((x: any, y: any) => (
// 					<Box cursor="pointer" key={y} maxW="10rem">
// 						<Link passHref href={`/guilds/${x.id}`}>
// 							<Box>
// 								<Image alt={x.name} src={x.icon || '/notfound.png'} />
// 								<Text className="text-center">{x.name}</Text>
// 							</Box>
// 						</Link>
// 					</Box>
// 				))}
// 			</Wrap>
// 		</div>
// 	);
// }

// export const getServerSideProps: GetServerSideProps<Props> = async function (
// 	ctx
// ) {
// 	const user = await parseUser(ctx);

// 	if (!user) {
// 		return {
// 			redirect: {
// 				destination: '/api/login',
// 				permanent: false,
// 			},
// 		};
// 	}

// 	let guilds = await axios.post('http://localhost:8080/guilds', user.guilds);

// 	return { props: { user, guilds: guilds.data } };
// };
