import { Entity, PrimaryColumn, Column, CreateDateColumn, UpdateDateColumn } from 'typeorm';

@Entity()
export class Task {
  @PrimaryColumn()
  taskId: number;

  @Column()
  creator: string;

  @Column()
  targetContract: string;

  @Column()
  triggerType: string;

  @Column('text')
  triggerData: string;

  @Column('bigint')
  reward: string;

  @Column('bigint')
  executeAfter: string;

  @Column({ default: 'PENDING' })
  status: string;

  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}
