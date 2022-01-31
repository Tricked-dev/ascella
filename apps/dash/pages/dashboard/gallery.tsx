/* eslint-disable jsx-a11y/alt-text */
import {
	Box,
	Button,
	Center,
	ComponentWithAs,
	Grid,
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
	Text,
	useDisclosure,
	grid,
	SimpleGrid,
	GridItem,
} from '@chakra-ui/react';
import PropTypes from 'prop-types';
import { useEffect, useState } from 'react';
import DashBoardItem from '../../components/DashboardItem';
import Layout from '../../components/DashboardLayout';
import fetch from '../../utils/fetch';
import withSession from '../../utils/withSession';
import isoFetch from 'isomorphic-fetch';

export default function Dashboard({ user, domains }: any) {
	const [current, setCurrent] = useState(0);
	const [images, setImages] = useState([] as Record<string, string>[]);

	async function update() {
		try {
			let r = await isoFetch('/api/gallery', {
				method: 'POST',
				body: JSON.stringify({
					skip: current,
				}),
			});

			setImages((await r.json()) || []);
			console.log(images);
		} catch (e) {
			console.log(e);
		}
	}

	useEffect(() => {
		update();
	}, []);

	return (
		<Layout>
			{/* <button onClicK={update}>Update</button> */}
			<SimpleGrid minChildWidth="300px" spacing="5px">
				{(Array.isArray(images) ? images : []).map((x, index) => {
					return (
						<GridItem
							key={index}
							border="2px"
							borderColor={'cyan.200'}
							w="300px"
							h="300px"
							justifyContent={'center'}
						>
							<Image
								w="auto"
								h="auto"
								src={`https://ascella.wtf/v2/ascella/view/${x.vanity}`}
							></Image>
						</GridItem>
					);
				})}
			</SimpleGrid>
			<Button
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

export const getServerSideProps = withSession(async function ({
	req,
	res,
}: any) {
	const user = req.session.get('user');

	if (user === undefined) {
		return {
			redirect: {
				destination: '/',
				permanent: false,
			},
			props: {},
		};
	}

	return {
		props: {
			user: req.session.get('user'),
		},
	};
});
