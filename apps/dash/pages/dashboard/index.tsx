import withSession from '../../utils/withSession';
import PropTypes from 'prop-types';
import Layout from '../../components/DashboardLayout';
import {
	Button,
	Grid,
	GridItem,
	Input,
	InputGroup,
	InputLeftAddon,
	Text,
} from '@chakra-ui/react';
import fetch from '../../utils/fetch';
import DashBoardItem from '../../components/DashboardItem';
export default function Dashboard({ user }: any) {
	async function downloadConfig() {
		let blob = await fetch(`/config?id=${user.id}&key=${user.key}`, user).then(
			(r) => r.blob()
		);
		let url = URL.createObjectURL(blob);

		const anchor = document.createElement('a');
		anchor.href = url;
		anchor.target = '_blank';
		anchor.download = 'config.sxcu';

		// Auto click on a element, trigger the file download
		anchor.click();

		// This is required
		URL.revokeObjectURL(url);
	}
	return (
		<Layout>
			<Grid
				templateRows="repeat(2, 1fr)"
				templateColumns="repeat(2, 1fr)"
				gridGap={'20px'}
			>
				<GridItem colSpan={2} textAlign={'center'}>
					<DashBoardItem>
						<Text fontSize={'xl'}>Welcome {user.name}</Text>
					</DashBoardItem>
				</GridItem>
				<GridItem colSpan={2}>
					<DashBoardItem>
						<Text fontSize={'xl'}>Need a new config?</Text>
						<Button
							variant={'outline'}
							colorScheme={'teal'}
							onClick={() => downloadConfig()}
						>
							Download Config
						</Button>
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

	return {
		props: { user: req.session.get('user') },
	};
});
