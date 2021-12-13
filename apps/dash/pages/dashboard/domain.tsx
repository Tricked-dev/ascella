/* eslint-disable jsx-a11y/alt-text */
import withSession from '../../utils/withSession';
import PropTypes from 'prop-types';
import Layout from '../../components/DashboardLayout';
import {
	Box,
	Button,
	ComponentWithAs,
	Grid,
	GridItem,
	GridItemProps,
	Input,
	Link,
	Text,
	Center,
	InputGroup,
	Image,
	InputLeftAddon,
	Select,
	Modal,
	ModalBody,
	ModalCloseButton,
	ModalContent,
	ModalFooter,
	ModalHeader,
	ModalOverlay,
	useDisclosure,
} from '@chakra-ui/react';
import fetch from '../../utils/fetch';
import { useState } from 'react';
import DashBoardItem from '../../components/DashboardItem';

export default function Dashboard({ user, domains }: any) {
	const { isOpen, onOpen, onClose } = useDisclosure();

	const [domain, setDomain] = useState(''),
		[subdomain, setSubdomain] = useState(''),
		[title, setTitle] = useState(''),
		[link, setLink] = useState(''),
		[url, setUrl] = useState(''),
		[description, setDescription] = useState(''),
		[color, setColor] = useState(''),
		[message, setMessage] = useState('');

	async function submitDomain() {
		onOpen();
		setMessage('Loading.......');
		let dd = `https://${
			subdomain?.length !== 0 ? `${subdomain}.` : ''
		}${domain}`;
		console.log({ key: user.key, id: user.id, domain: dd });
		let r = await fetch(
			'/domain',
			user,
			JSON.stringify({
				key: user.key,
				id: user.id,
				domain: dd,
			}),
			'POST'
		);
		if (r.ok) {
			user.domain = dd;
			setMessage(`Successfully updated your domain to ${dd}`);
		} else {
			setMessage(`Failed to update your domain ${await r.text()}`);
		}
	}
	async function submitEmbed() {
		onOpen();
		setMessage('Loading.......');

		let r = await fetch(
			'/embed',
			user,
			JSON.stringify({
				title: title,
				link: link,
				url: url,
				description: description,
				color: color,
			}),
			'POST'
		);
		if (r.ok) {
			setMessage(
				`Successfully updated your embed check it out by making a new screenshot!`
			);
		} else {
			setMessage(`Failed to update your domain ${await r.text()}`);
		}
	}

	return (
		<Layout>
			<Modal isOpen={isOpen} onClose={onClose}>
				<ModalOverlay />
				<ModalContent>
					<ModalHeader>updating settings</ModalHeader>
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
				gridGap={'20px'}
			>
				<GridItem>
					<DashBoardItem
						display={'flex'}
						justifyContent={'center'}
						alignItems={'center'}
					>
						<Text fontSize={'lg'}>
							currently uploading to{' '}
							<Link href={user.domain}>{user.domain}</Link>
						</Text>
					</DashBoardItem>
				</GridItem>
				<GridItem />

				<GridItem>
					<DashBoardItem>
						<Text fontSize={'lg'}>Select a new domain</Text>

						<InputGroup colorScheme={'teal'}>
							<Input
								colorScheme={'teal'}
								onChange={(e) => setSubdomain(e.target.value)}
								width="30%"
								placeholder="subdomain"
							/>
							<Select
								colorScheme={'teal'}
								onChange={(e) => setDomain(e.target.value)}
								placeholder="Select a domain"
							>
								{domains.map((x: any, y: any) => {
									return (
										<option key={y} value={x}>
											{x}
										</option>
									);
								})}
							</Select>
						</InputGroup>
						<Button onClick={submitDomain} colorScheme={'teal'}>
							Submit
						</Button>
					</DashBoardItem>
				</GridItem>

				<GridItem colSpan={2}>
					<DashBoardItem>
						<Text fontSize={'lg'}>Change embed settings</Text>
						<Input
							onChange={(x) => setTitle(x.target.value)}
							placeholder="title"
						></Input>
						<Input
							onChange={(x) => setLink(x.target.value)}
							placeholder="link"
							disabled={true}
						></Input>
						<Input
							onChange={(x) => setUrl(x.target.value)}
							placeholder="url"
							disabled={true}
						></Input>
						<Input
							onChange={(x) => setDescription(x.target.value)}
							placeholder="description"
						></Input>
						<Input
							onChange={(x) => setColor(x.target.value)}
							placeholder="color"
						></Input>
						<Button onClick={submitEmbed} colorScheme={'teal'}>
							Submit
						</Button>
						<Box padding="2rem" background="#2c2f33" maxW={'70%'}>
							<Link fontSize={'1.3rem'} color="blue.500" href="#">
								{user.domain}/1234
							</Link>
							<Box height="20rem" width="auto">
								{!title && !description && (
									<Image
										height="100%"
										width="auto"
										src="https://cdn.discordapp.com/attachments/748957504666599507/905949227753025566/Untitled91.png"
									></Image>
								)}
								{(title || description) && (
									<Box
										height="100%"
										width="auto"
										borderLeftRadius={'10x'}
										borderBottom={'10px'}
										borderLeftWidth={'10px'}
										rounded="10px"
										borderLeftColor={color}
										padding="0.4rem"
										background="#23272a"
										// width="100%"
										maxW={'80%'}
									>
										<Text fontSize={'1.3rem'} color="blue.500">
											<Link href="https://ascella.host">{title}</Link>
										</Text>
										<Text paddingTop={'0.2rem'} color="gray.300">
											{description}
										</Text>
										<Image
											height="88%"
											width="auto"
											src="https://cdn.discordapp.com/attachments/748957504666599507/905949227753025566/Untitled91.png"
										></Image>
									</Box>
								)}
							</Box>
						</Box>
					</DashBoardItem>
				</GridItem>
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
