import { Box } from "@chakra-ui/react";

export default function DashBoardItem({ children, ...props }: any) {
    return (
        <Box
            background="gray.600"
            h="100%"
            w="100%"
            boxShadow={"md"}
            padding={"2"}
            borderRadius={"5px"}
            rounded="lg"
            {...props}
        >
            {children}
        </Box>
    );
}
