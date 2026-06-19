import { Entity, PrimaryGeneratedColumn, Column, CreateDateColumn } from 'typeorm';

@Entity()
export class Execution {
  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  taskId: number;

  @Column()
  keeperAddress: string;

  @Column()
  txHash: string;

  @Column({ default: 'SUCCESS' })
  status: string;

  @CreateDateColumn()
  executedAt: Date;
}
