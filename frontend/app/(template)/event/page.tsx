"use client";

import { useState } from "react";
import Table from "@/components/ui/Table";
import Button from "@/components/ui/Button";
import { useEvents } from "@/hooks/useEvents";
import type { Event } from "@/types/Event";
import type { Column } from "@/components/ui/Table";

const formatDate = (dateStr: string) =>
  new Date(dateStr).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });

const columns: Column<Event>[] = [
  { header: "ID", accessor: "id", className: "w-16 font-mono text-gray-400" },
  { header: "Name", accessor: "nom", className: "font-medium text-gray-900" },
  { header: "Location", accessor: "lieu" },
  {
    header: "Date",
    accessor: (row) => formatDate(row.date),
  },
  {
    header: "Created",
    accessor: (row) => formatDate(row.created_at),
    className: "text-gray-400",
  },
  {
    header: "Actions",
    accessor: (row) => (
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="sm" onClick={() => alert(`Edit: ${row.id}`)}>
          Edit
        </Button>
        <Button variant="danger" size="sm" onClick={() => alert(`Delete: ${row.id}`)}>
          Delete
        </Button>
      </div>
    ),
  },
];

export default function EventPage() {
  const { data: events = [], isLoading, isError, refetch } = useEvents();
  const [search, setSearch] = useState("");

  const filtered = events.filter(
    (e) =>
      e.nom.toLowerCase().includes(search.toLowerCase()) ||
      e.lieu.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Events</h1>
          <p className="text-sm text-gray-500 mt-0.5">
            Manage all your events in one place.
          </p>
        </div>
        <div className="flex gap-2">
          <Button
            variant="secondary"
            size="sm"
            onClick={() => refetch()}
            isLoading={isLoading}
          >
            Refresh
          </Button>
          <Button
            variant="primary"
            size="sm"
            onClick={() => alert("Open create modal")}
          >
            + New Event
          </Button>
        </div>
      </div>

      {/* Search */}
      <input
        type="text"
        value={search}
        onChange={(e) => setSearch(e.target.value)}
        placeholder="Search by name or location…"
        className="w-full max-w-xs px-4 py-2 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
      />

      {/* Error banner */}
      {isError && (
        <div className="rounded-md bg-red-50 border border-red-200 px-4 py-3 text-sm text-red-700">
          Failed to load events. Check that the backend is running and try again.
        </div>
      )}

      {/* Table */}
      <Table
        columns={columns}
        data={filtered}
        keyExtractor={(row) => row.id}
        isLoading={isLoading}
        emptyMessage="No events match your search."
      />
    </div>
  );
}
