import React from "react";
import { Landmark, Search, ChevronDown, LayoutGrid, List } from "lucide-react";
import FeaturedGoalCard from "@/app/components/dashboard/FeaturedGoalCard";
import GoalOverviewCard from "@/app/components/dashboard/GoalOverviewCard";
import GoalCard, { type GoalStatus } from "@/app/components/dashboard/GoalCard";

export const metadata = { title: "Savings Pools – Nestera" };

export default function GoalBasedSavingsPage() {
  const goals: Array<{
    title: string;
    category: string;
    currentAmount: number;
    targetAmount: number;
    targetDate: string;
    status: GoalStatus;
  }> = [
    {
      title: "Emergency Fund",
      category: "Security",
      currentAmount: 5200,
      targetAmount: 10000,
      targetDate: "Sep 30, 2025",
      status: "On Track",
    },
    {
      title: "New Laptop",
      category: "Tech",
      currentAmount: 950,
      targetAmount: 1800,
      targetDate: "Dec 20, 2025",
      status: "At Risk",
    },
    {
      title: "Home Down Payment",
      category: "Housing",
      currentAmount: 14200,
      targetAmount: 25000,
      targetDate: "Jun 01, 2026",
      status: "On Track",
    },
    {
      title: "Conference Trip",
      category: "Travel",
      currentAmount: 1200,
      targetAmount: 2500,
      targetDate: "Oct 18, 2025",
      status: "Paused",
    },
  ];

  return (
    <div className="w-full max-w-7xl mx-auto pb-20">
      {/* Page Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-10">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-linear-to-b from-[#063d3d] to-[#0a6f6f] flex items-center justify-center text-cyan-400 shadow-[0_8px_20px_rgba(6,61,61,0.3)]">
            <Landmark size={24} />
          </div>
          <div>
            <h1 className="text-3xl font-bold text-white m-0 tracking-tight">
              Savings Pools
            </h1>
            <p className="text-[#5e8c96] text-sm md:text-base m-0 mt-1">
              Discover and manage savings pools across supported assets.
            </p>
          </div>
        </div>

        {/* View Toggles & Actions */}
        <div className="flex items-center gap-3">
          <div className="flex bg-[#0e2330] p-1 rounded-xl border border-white/5">
            <button className="p-2 rounded-lg bg-cyan-500/10 text-cyan-400 shadow-sm">
              <LayoutGrid size={18} />
            </button>
            <button className="p-2 rounded-lg text-[#5e8c96] hover:text-white transition-colors">
              <List size={18} />
            </button>
          </div>
          <button className="px-5 py-2.5 bg-cyan-500 hover:bg-cyan-400 text-[#061a1a] font-bold rounded-xl transition-all shadow-lg active:scale-95">
            Create New Goal
          </button>
        </div>
      </div>

      {/* Search & Filters Row */}
      <div className="flex flex-wrap items-center gap-4 mb-8">
        <div className="relative flex-1 min-w-[300px]">
          <Search
            className="absolute left-4 top-1/2 -translate-y-1/2 text-[#5e8c96]"
            size={18}
          />
          <input
            type="text"
            placeholder="Search pools by name, strategy, or provider..."
            className="w-full bg-[#0e2330] border border-white/5 rounded-xl py-3 pl-12 pr-4 text-white placeholder:text-[#4e7a86] focus:outline-hidden focus:border-cyan-500/50 transition-colors"
          />
        </div>

        <div className="flex flex-wrap items-center gap-3">
          {[
            { label: "Asset: All", active: true },
            { label: "Risk: All Levels", active: false },
            { label: "Sort by: APY", active: false },
          ].map((filter, i) => (
            <button
              key={i}
              className={`flex items-center gap-2 px-4 py-3 rounded-xl border transition-all ${
                filter.active
                  ? "bg-cyan-500/5 border-cyan-500/20 text-cyan-400"
                  : "bg-[#0e2330] border-white/5 text-[#5e8c96] hover:border-white/10 hover:text-white"
              }`}
            >
              <span className="text-sm font-medium">{filter.label}</span>
              <ChevronDown size={14} opacity={0.7} />
            </button>
          ))}
        </div>
      </div>

      {/* Goal Overview Card */}
      <div className="mb-10">
        <GoalOverviewCard
          title="Emergency Fund"
          status="On Track"
          description="Building a 6-month emergency safety net to cover unexpected expenses and provide financial security."
          percentage={52}
          savedAmount={5200}
          targetAmount={10000}
          monthlyContribution={400}
          deadline="Sep 30, 2025"
        />
      </div>

      {/* Featured Goal Card */}
      <div className="mb-12">
        <FeaturedGoalCard
          title="Summer Vacation Fund"
          category="Travel"
          currentAmount={7800}
          targetAmount={10000}
          targetDate="Aug 31, 2024"
          status="On Track"
          percentage={78}
          motivationalText="You're 78% of the way there! Keep it up!"
        />
      </div>

      {/* Section Divider / Goals Grid Placeholder */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-xl font-bold text-white m-0">All Goals</h3>
        <span className="text-[#5e8c96] text-sm">
          Showing {goals.length} goals
        </span>
      </div>

      {/* Goals Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {goals.map((goal) => (
          <GoalCard
            key={goal.title}
            title={goal.title}
            category={goal.category}
            currentAmount={goal.currentAmount}
            targetAmount={goal.targetAmount}
            targetDate={goal.targetDate}
            status={goal.status}
            href="#"
          />
        ))}

        <a
          href="/savings/create-goal"
          className="aspect-[4/3] rounded-3xl border-2 border-dashed border-white/5 flex flex-col items-center justify-center text-[#4e7a86] bg-white/[0.01] hover:bg-white/[0.02] transition-colors cursor-pointer group"
        >
          <div className="w-12 h-12 rounded-full bg-white/5 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
            <span className="text-3xl">+</span>
          </div>
          <p className="font-semibold m-0">Create another goal</p>
        </a>
      </div>
    </div>
  );
}
