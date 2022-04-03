/* eslint-disable jsx-a11y/alt-text */
import {
  Box,
  Button,
  Center,
  ComponentWithAs,
  Grid,
  grid,
  GridItem,
  GridItemProps,
  Image,
  Input,
  InputGroup,
  InputLeftAddon,
  Link,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  Select,
  SimpleGrid,
  Tag,
  Text,
  useDisclosure,
  Wrap,
} from "@chakra-ui/react";
import isoFetch from "isomorphic-fetch";
import PropTypes from "prop-types";
import { useEffect, useState } from "react";
import DashBoardItem from "../../components/DashboardItem";
import Layout from "../../components/DashboardLayout";
import fetch from "../../utils/fetch";
import withSession from "../../utils/withSession";

export default function Dashboard({ user, domains }: any) {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const [current, setCurrent] = useState(0);
  const [images, setImages] = useState([] as Record<string, string>[]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [delay, setDelay] = useState(0);
  async function update() {
    setLoading(true);
    try {
      console.log(current);
      setDelay(0);
      let r = await isoFetch("/api/gallery", {
        method: "POST",
        body: JSON.stringify({
          skip: current,
        }),
      });
      const json = await r.json();
      if (json.error) {
        setError(`${json.error} Please slow down`);
        onOpen();
      } else {
        setImages(json || []);
        setError("");
      }
      console.log(images);
    } catch (e) {
      console.log(e);
    }
    setLoading(false);
  }

  useEffect(() => {
    update();
  }, []);

  return (
    <Layout>
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Failed to fetch images</ModalHeader>
          <ModalCloseButton />
          <ModalBody>{error}</ModalBody>

          <ModalFooter>
            <Button colorScheme="blue" mr={3} onClick={onClose}>
              Close
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
      <SimpleGrid minChildWidth="300px" spacing="5px">
        {!loading
          && (Array.isArray(images) ? images : []).map((x, index) => {
            return (
              <GridItem
                key={index}
                w="300px"
                h="300px"
                justifyContent={"center"}
              >
                <Image
                  objectFit={"contain"}
                  src={`https://ascella.wtf/v2/ascella/view/${x.vanity}`}
                  border="2px"
                  borderColor={"cyan.200"}
                  onError={(_) => {
                    setError(
                      "Failed to load a image this is likely due a ratelimit issue",
                    );
                    onOpen();
                  }}
                >
                </Image>
                <Wrap gap="2" pt="2">
                  <a href={`https://ascella.host/${x.vanity}`}>
                    <Tag>{x.vanity}</Tag>
                  </a>
                  <Tag>{x.id}</Tag>
                </Wrap>
              </GridItem>
            );
          })}
      </SimpleGrid>
      {current !== 0 && !loading && !isOpen && (
        <Button
          isLoading={loading}
          onClick={async () => {
            setCurrent(current - 20);
            await update();
          }}
        >
          Back
        </Button>
      )}

      <Button
        isLoading={loading}
        onClick={async () => {
          setCurrent(current + 20);
          await update();
        }}
      >
        Next
      </Button>
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
    props: {
      user: req.session.get("user"),
    },
  };
});
