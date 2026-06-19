import { Controller, Get, Param } from '@nestjs/common';
import { KeepersService } from './keepers.service';

@Controller('keepers')
export class KeepersController {
  constructor(private readonly keepers: KeepersService) {}

  @Get()
  findAll() { return this.keepers.findAll(); }

  @Get(':address')
  findOne(@Param('address') address: string) { return this.keepers.findOne(address); }
}
