import { Entity, PrimaryColumn, Column, UpdateDateColumn } from 'typeorm';

@Entity()
export class Keeper {
  @PrimaryColumn()
  address: string;

  @Column('bigint', { default: '0' })
  stake: string;

  @Column({ default: 100 })
  reputation: number;

  @Column({ default: 0 })
  successfulExecutions: number;

  @Column({ default: 0 })
  failedExecutions: number;

  @UpdateDateColumn()
  updatedAt: Date;
}
