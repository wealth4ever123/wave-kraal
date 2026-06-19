import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Keeper } from './keeper.entity';

@Injectable()
export class KeepersService {
  constructor(@InjectRepository(Keeper) private keepers: Repository<Keeper>) {}

  findAll() { return this.keepers.find({ order: { reputation: 'DESC' } }); }

  findOne(address: string) { return this.keepers.findOneBy({ address }); }

  upsert(data: Partial<Keeper>) { return this.keepers.save(data); }
}
