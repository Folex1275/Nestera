import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  CreateDateColumn,
} from 'typeorm';

@Entity('dead_letter_events')
export class DeadLetterEvent {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  /** Ledger sequence number at the time of failure */
  @Column({ type: 'bigint' })
  ledgerSequence: number;

  /** Raw Soroban event payload serialised to JSON string */
  @Column({ type: 'text' })
  rawEvent: string;

  /** Error message captured from the thrown exception */
  @Column({ type: 'text', nullable: true })
  errorMessage: string;

  @CreateDateColumn()
  createdAt: Date;
}
