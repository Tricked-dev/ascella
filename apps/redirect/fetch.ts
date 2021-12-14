fetch('http://localhost:3000/yHOLpH2', {
	headers: {
		'user-agent': 'discord Discord',
	},
})
	.then((r) => r.text())
	.then(console.log);
