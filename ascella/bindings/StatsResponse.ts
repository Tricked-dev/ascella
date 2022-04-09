import type { Embeds } from "./Embeds";

export interface StatsResponse {
  user_name: string;
  user_id: number;
  id: number;
  redirect?: string;
  content_type: string;
  image_size?: string;
  embed?: Embeds;
}
