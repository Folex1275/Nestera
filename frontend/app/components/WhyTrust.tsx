import React from 'react';
import { CheckCircle2 } from 'lucide-react';

const trustItems = [
  {
    title: 'Transparent',
    description:
      'Every transaction is verifiable on the Stellar network. No hidden logic.',
  },
  {
    title: 'Non-custodial',
    description:
      'You hold the keys. We never touch your funds or control your assets.',
  },
  {
    title: 'Low Fees',
    description:
      "Pennies per transaction, thanks to Soroban's efficiency.",
  },
  {
    title: 'Fast Finality',
    description:
      'Settlement in seconds, not days. Access your funds when you need them.',
  },
];

const WhyTrust: React.FC = () => {
  return (
    <section
      className="w-full bg-[#052022] px-6 py-16 md:px-12 md:py-20 lg:px-16"
      aria-labelledby="why-trust-title"
    >
      <div className="mx-auto grid max-w-6xl gap-10 md:grid-cols-[1.2fr_0.8fr] md:items-center">
        <div>
          <h2
            id="why-trust-title"
            className="mb-10 text-[clamp(1.75rem,4vw,2.5rem)] font-bold tracking-[-0.02em] text-white"
          >
            Why Trust Nestera?
          </h2>

          <div className="flex flex-col gap-6">
            {trustItems.map((item) => (
              <article key={item.title} className="flex items-start gap-4">
                <CheckCircle2
                  className="mt-1 shrink-0 text-[#1ABC9C]"
                  size={22}
                  aria-hidden
                />
                <div>
                  <h3 className="text-lg font-bold text-white">{item.title}</h3>
                  <p className="text-sm leading-relaxed text-[rgba(180,210,210,0.85)]">
                    {item.description}
                  </p>
                </div>
              </article>
            ))}
          </div>
        </div>

        <div className="relative hidden h-64 md:block" aria-hidden>
          <div className="absolute left-8 top-4 h-40 w-40 rotate-12 rounded-3xl border border-[#1ABC9C]/25 bg-[#1ABC9C]/10" />
          <div className="absolute left-24 top-20 h-36 w-36 -rotate-6 rounded-3xl border border-[#1ABC9C]/20 bg-[#1ABC9C]/8" />
        </div>
      </div>
    </section>
  );
};

export default WhyTrust;
