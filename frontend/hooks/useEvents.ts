import { useQuery } from "@tanstack/react-query";
import type { Event } from "@/types/Event";
import { apiClient } from "@/lib/apiClient";

async function fetchEvents(): Promise<Event[]> {
  const { data } = await apiClient.get<Event[]>("/event");
  return data;
}

export function useEvents() {
  return useQuery<Event[], Error>({
    queryKey: ["events"],
    queryFn: fetchEvents,
  });
}
