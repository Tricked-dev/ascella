import watchMedia from "svelte-media";

const mediaqueries = {
  tablet: "(min-width: 481px) and (max-width: 1280px)",
  mobile: "(max-width: 480px)",
  laptop: "(min-width: 1281px)",
};

export const media = watchMedia(mediaqueries);
