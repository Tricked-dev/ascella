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
	GridItem,
} from '@chakra-ui/react';
import PropTypes from 'prop-types';
import { useEffect, useState } from 'react';
import DashBoardItem from '../../components/DashboardItem';
import Layout from '../../components/DashboardLayout';
import fetch from '../../utils/fetch';
import withSession from '../../utils/withSession';

export default function Dashboard({ user, domains }: any) {
	const [current, setCurrent] = useState(0);
	const [images, setImages] = useState([] as Record<string, string>[]);

	async function update() {
		try {
			let r = await fetch(
				'/images',
				user,
				JSON.stringify({
					skip: current,
				}),
				'POST'
			);
			setImages(await r.json());
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
			<Grid>
				{images.map((x, index) => {
					return (
						<GridItem key={index} border="2px" borderColor={'cyan.200'}>
							<Image src={`https://ascella.host/${x.vanity}`}></Image>
						</GridItem>
					);
				})}
			</Grid>
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
	let domains = await fetch('/domains', {}, undefined).then(
		(r: any) => r.json() as any
	);

	return {
		props: {
			user: req.session.get('user'),
			domains: domains.map((x: any) => x.domain),
		},
	};
});
