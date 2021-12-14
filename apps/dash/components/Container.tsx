import { Box, Link } from "@chakra-ui/react";
const bottomLinks = [
    [
        {
            href: "/docs/faq",
            a: "Faq",
        },
        {
            href: "/docs/rules",
            a: "Rules",
        },
        {
            href: "/docs/privacy",
            a: "Privacy",
        },
    ],
    [
        {
            href: "https://github.com/Tricked-dev/uploader",
            a: "Github",
        },
    ],
];
export function Container({ children }: any) {
    return (
        <>
            <Box>{children}</Box>
            <Box display="flex" bg="red.400" flex="auto" zIndex={"toast"}>
                {bottomLinks.map((x, y) => {
                    return (
                        <Box key={y}>
                            <ul>
                                {x.map((link, y) => {
                                    return (
                                        <li key={y}>
                                            <Link padding="4px" py={"2"} href={link.href}>
                                                {link.a}
                                            </Link>
                                        </li>
                                    );
                                })}
                            </ul>
                        </Box>
                    );
                })}
            </Box>
        </>
    );
}
