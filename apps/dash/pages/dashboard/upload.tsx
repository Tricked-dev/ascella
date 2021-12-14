/* eslint-disable react/no-children-prop */
/* eslint-disable jsx-a11y/alt-text */
import {
    Button,
    Grid,
    GridItem,
    Image,
    Input,
    InputGroup,
    InputLeftAddon,
    Modal,
    ModalBody,
    ModalCloseButton,
    ModalContent,
    ModalFooter,
    ModalHeader,
    ModalOverlay,
    Text,
    useDisclosure,
} from "@chakra-ui/react";
import PropTypes from "prop-types";
import { useState } from "react";
import DashBoardItem from "../../components/DashboardItem";
import Layout from "../../components/DashboardLayout";
import fetch from "../../utils/fetch";
import withSession from "../../utils/withSession";
export default function Dashboard({ user }: any) {
    const { isOpen, onOpen, onClose } = useDisclosure();

    const [selectedFile, setSelectedFile]: any = useState(null);
    const [message, setMessage]: any = useState("");
    const [response, setResponse]: any = useState(null);
    const [raw, setRaw]: any = useState(null);
    const [redirect, setRedirect]: any = useState(null);
    const [vanity, setVanity]: any = useState(null);
    const [done, setDone]: any = useState(null);
    async function upload() {
        let part = new FormData();
        // @ts-ignore
        part.set("image", selectedFile);
        let r = await fetch("/upload", user, part, "POST");
        if (r.ok) {
            let res = await r.json();
            setResponse(res.url);
            setRaw(res.raw);
        } else {
            setMessage(await r.text());
            onOpen();
        }
    }
    async function createRedirect() {
        let r = await fetch(
            "/redirect",
            user,
            JSON.stringify({
                to: redirect,
                vanity: vanity,
            }),
            "POST",
        );
        if (r.ok) {
            let res = await r.json();
            setDone(true);
        } else {
            setMessage(await r.text());
            onOpen();
        }
    }
    return (
        <Layout>
            <Modal isOpen={isOpen} onClose={onClose}>
                <ModalOverlay />
                <ModalContent>
                    <ModalHeader>Failed to upload</ModalHeader>
                    <ModalCloseButton />
                    <ModalBody>{message}</ModalBody>

                    <ModalFooter>
                        <Button colorScheme="blue" mr={3} onClick={onClose}>
                            Close
                        </Button>
                    </ModalFooter>
                </ModalContent>
            </Modal>
            <Grid
                templateRows="repeat(2, 1fr)"
                templateColumns="repeat(2, 1fr)"
                gridGap={"20px"}
            >
                <GridItem>
                    <DashBoardItem>
                        {!response && (
                            <>
                                Upload a image
                                <Input
                                    onChange={(e: any) => setSelectedFile(e.target.files[0])}
                                    type="file"
                                >
                                </Input>
                                <Button onClick={() => upload()} variant={"solid"}>
                                    Upload image
                                </Button>
                            </>
                        )}
                        {response && (
                            <>
                                <a target="_blank" rel="noopener noreferrer" href={response}>
                                    <Image src={raw}></Image>
                                </a>
                            </>
                        )}
                    </DashBoardItem>
                </GridItem>
                <GridItem>
                    <DashBoardItem>
                        Create a vanity
                        <InputGroup size="sm">
                            <InputLeftAddon children={`${user.domain}/`} />
                            <Input
                                onChange={(e) => setVanity(e.target.value)}
                                placeholder="vanity"
                            />
                        </InputGroup>
                        <Input
                            onChange={(e) => setRedirect(e.target.value)}
                            placeholder="redirect to"
                        >
                        </Input>
                        {!done && (
                            <Button onClick={() => createRedirect()} variant={"solid"}>
                                Create redirect
                            </Button>
                        )}
                        {done && (
                            <>
                                <Text color="blue.800">
                                    Vanity created{" "}
                                    <a href={`${user.domain}/${vanity}`}>
                                        {user.domain}/{vanity}
                                    </a>
                                </Text>
                            </>
                        )}
                    </DashBoardItem>
                </GridItem>
            </Grid>
        </Layout>
    );
}

export const getServerSideProps = withSession(async function({
    req,
    res,
}: any) {
    const user = req.session.get("user");

    if (user === undefined) {
        return {
            redirect: {
                destination: "/",
                permanent: false,
            },
            props: {},
        };
    }

    return {
        props: { user: req.session.get("user") },
    };
});
